use crate::{
    api::{PagedQuery, PagedResponse},
    convert_mon_stats, AppState, SingleMonStats,
};
use actix_files::NamedFile;
use actix_web::{get, web, Responder, Result};
use all_battle_core::{
    models,
    schema::{self},
};
use chrono::prelude::*;
use diesel::dsl::count_distinct;
use diesel::{dsl::sum, prelude::*};
use serde_derive::{Deserialize, Serialize};

#[get("/video/{id}")]
async fn video(data: web::Data<AppState>, battle: web::Path<i32>) -> Result<NamedFile> {
    use schema::battles;

    let battle_id = battle.into_inner();

    let video_path = {
        let conn = &mut data.conn_pool.get().unwrap();

        let video_path: String = match battles::table
            .find(battle_id)
            .select(battles::video_path)
            .first::<String>(conn)
        {
            Ok(v) => v,
            Err(_) => return Err(actix_web::error::ErrorNotFound("Battle not found")),
        };

        video_path
    };

    Ok(NamedFile::open(
        data.video_path_root
            .join(video_path)
            .to_str()
            .unwrap()
            .to_string(),
    )?)
}

#[derive(Serialize)]
struct BattleStatsResponse {
    pub battle_id: i32,
    pub player_mons: Vec<SingleMonStats>,
    pub opponent_mons: Vec<SingleMonStats>,
}

#[get("/stats/{id}")]
async fn stats(data: web::Data<AppState>, battle: web::Path<i32>) -> impl Responder {
    use schema::battles;
    use schema::trainer_mon_stats;

    let battle_id = battle.into_inner();

    let (mons, player_perspective, opponent_perspective) = {
        let conn = &mut data.conn_pool.get().unwrap();

        let mons: Vec<models::TrainerMonStat> = trainer_mon_stats::table
            .filter(trainer_mon_stats::battle_id.eq(battle_id))
            .load::<models::TrainerMonStat>(conn)
            .expect("Error loading player mon stats");

        let (player_perspective, opponent_perspective) = match battles::table
            .filter(battles::id.eq(battle_id))
            .select((battles::player_perspective, battles::opponent_perspective))
            .first::<(i32, i32)>(conn)
        {
            Ok((x, y)) => (x, y),
            Err(_) => return Err(actix_web::error::ErrorNotFound("Battle not found")),
        };

        (mons, player_perspective, opponent_perspective)
    };

    let mut player_mons = vec![];
    let mut opponent_mons = vec![];
    for mon in mons {
        if mon.trainer_id == player_perspective {
            player_mons.push(mon);
        } else if mon.trainer_id == opponent_perspective {
            opponent_mons.push(mon);
        }
    }

    let player_mons = convert_mon_stats(&mut data.conn_pool.get().unwrap(), player_mons);
    let opponent_mons = convert_mon_stats(&mut data.conn_pool.get().unwrap(), opponent_mons);

    Ok(web::Json(BattleStatsResponse {
        battle_id,
        player_mons,
        opponent_mons,
    }))
}

macro_rules! build_battles_query {
    ($query_args:expr) => {{
        use schema::trainer_mon_stats_moves;

        let mut battles_query = battles::table.into_boxed();

        if let Some(min_length) = $query_args.min_length_seconds {
            battles_query = battles_query.filter(battles::duration.ge(min_length));
        }

        if let Some(max_length) = $query_args.max_length_seconds {
            battles_query = battles_query.filter(battles::duration.le(max_length));
        }

        if let Some(seed) = &$query_args.seed {
            if let Ok(seed) = seed.parse::<i32>() {
                battles_query = battles_query.filter(battles::seed.eq(seed));
            }
        }

        if let Some(player_perspective_won) = &$query_args.player_perspective_won {
            if player_perspective_won == "true" {
                battles_query = battles_query.filter(battles::player_perspective_won.eq(true));
            } else if player_perspective_won == "false" {
                battles_query = battles_query.filter(battles::player_perspective_won.eq(false));
            }
        }

        if $query_args.mons_included.len() > 0 {
            let mons_included: Vec<i32> = $query_args
                .mons_included
                .split(',')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .collect();

            if mons_included.len() > 0 {
                let mon_count = mons_included.len() as i64;

                battles_query = battles_query.filter(
                    battles::id.eq_any(
                        trainer_mon_stats::table
                            .select(trainer_mon_stats::battle_id)
                            .filter(trainer_mon_stats::mon_id.eq_any(mons_included))
                            .filter(trainer_mon_stats::times_released.gt(0))
                            .group_by(trainer_mon_stats::battle_id)
                            .having(count_distinct(trainer_mon_stats::mon_id).eq(mon_count)),
                    ),
                );
            }
        }

        if $query_args.mons_excluded.len() > 0 {
            let mons_excluded: Vec<i32> = $query_args
                .mons_excluded
                .split(',')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .collect();

            if mons_excluded.len() > 0 {
                battles_query = battles_query.filter(
                    battles::id.ne_all(
                        trainer_mon_stats::table
                            .select(trainer_mon_stats::battle_id)
                            .filter(trainer_mon_stats::mon_id.eq_any(mons_excluded))
                            .filter(trainer_mon_stats::times_released.gt(0))
                            .group_by(trainer_mon_stats::battle_id),
                    ),
                );
            }
        }

        if $query_args.moves_used.len() > 0 {
            let moves_used: Vec<i32> = $query_args
                .moves_used
                .split(',')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .collect();

            if moves_used.len() > 0 {
                let move_count = moves_used.len() as i64;

                battles_query = battles_query.filter(
                    battles::id.eq_any(
                        trainer_mon_stats_moves::table
                            .inner_join(
                                trainer_mon_stats::table.on(trainer_mon_stats::id
                                    .eq(trainer_mon_stats_moves::trainer_mon_stats_id)),
                            )
                            .select(trainer_mon_stats::battle_id)
                            .filter(trainer_mon_stats_moves::move_id.eq_any(moves_used))
                            .filter(trainer_mon_stats_moves::times_used.gt(0))
                            .group_by(trainer_mon_stats::battle_id)
                            .having(
                                count_distinct(trainer_mon_stats_moves::move_id).eq(move_count),
                            ),
                    ),
                );
            }
        }

        if $query_args.moves_not_used.len() > 0 {
            let moves_not_used: Vec<i32> = $query_args
                .moves_not_used
                .split(',')
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .collect();

            if moves_not_used.len() > 0 {
                battles_query = battles_query.filter(
                    battles::id.ne_all(
                        trainer_mon_stats_moves::table
                            .inner_join(
                                trainer_mon_stats::table.on(trainer_mon_stats::id
                                    .eq(trainer_mon_stats_moves::trainer_mon_stats_id)),
                            )
                            .select(trainer_mon_stats::battle_id)
                            .filter(trainer_mon_stats_moves::move_id.eq_any(moves_not_used))
                            .filter(trainer_mon_stats_moves::times_used.gt(0))
                            .group_by(trainer_mon_stats::battle_id),
                    ),
                );
            }
        }

        let order_by_descending = $query_args.order_by_descending.unwrap_or(false);

        match $query_args
            .order_by_field
            .clone()
            .unwrap_or_default()
            .as_str()
        {
            "duration" => {
                if order_by_descending {
                    battles_query = battles_query.order(battles::duration.desc());
                } else {
                    battles_query = battles_query.order(battles::duration.asc());
                }
            }
            _ => {
                if order_by_descending {
                    battles_query = battles_query.order(battles::id.desc());
                } else {
                    battles_query = battles_query.order(battles::id.asc());
                }
            }
        }

        battles_query
    }};
}

#[derive(Serialize)]
struct BattleSearchEntry {
    pub battle_id: i32,
    pub series_id: i32,
    pub duration_seconds: i32,
    pub completed_at: String,
    pub seed: i32,
    pub player_perspective: i32,
    pub opponent_perspective: i32,
    pub player_perspective_won: bool,
}

fn max_i64() -> i64 {
    i64::max_value()
}

#[derive(Deserialize)]
struct SearchQuery {
    pub min_length_seconds: Option<i32>,
    pub max_length_seconds: Option<i32>,
    pub seed: Option<String>,
    pub player_perspective_won: Option<String>,
    // Trainer stats part
    #[serde(default)]
    pub min_kos: i64,
    #[serde(default = "max_i64")]
    pub max_kos: i64,
    #[serde(default)]
    pub min_damage_dealt: i64,
    #[serde(default = "max_i64")]
    pub max_damage_dealt: i64,
    #[serde(default)]
    pub mons_included: String,
    #[serde(default)]
    pub mons_excluded: String,
    #[serde(default)]
    pub moves_used: String,
    #[serde(default)]
    pub moves_not_used: String,

    pub order_by_field: Option<String>,
    pub order_by_descending: Option<bool>,
}

#[get("/search")]
async fn search(
    data: web::Data<AppState>,
    search_query: web::Query<SearchQuery>,
    page_query: web::Query<PagedQuery>,
) -> impl Responder {
    use schema::battles;
    use schema::trainer_mon_stats;

    let page = page_query.into_inner();
    let query_args = search_query.into_inner();

    let battles_query = build_battles_query!(query_args);
    let battles_query_count = build_battles_query!(query_args);

    let (total, battles) = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<(i32, i32, i32, i32, i32, i32, i64, bool)> = battles_query
            .filter(
                battles::id.eq_any(
                    trainer_mon_stats::table
                        .group_by(trainer_mon_stats::battle_id)
                        .select(trainer_mon_stats::battle_id)
                        .having(
                            sum(trainer_mon_stats::murders)
                                .ge(query_args.min_kos)
                                .and(sum(trainer_mon_stats::murders).le(query_args.max_kos))
                                .and(
                                    sum(trainer_mon_stats::damage_dealt)
                                        .ge(query_args.min_damage_dealt)
                                        .and(
                                            sum(trainer_mon_stats::damage_dealt)
                                                .le(query_args.max_damage_dealt),
                                        ),
                                ),
                        ),
                ),
            )
            .offset(page.offset)
            .limit(page.limit)
            .select((
                battles::id,
                battles::series_id,
                battles::duration,
                battles::seed,
                battles::player_perspective,
                battles::opponent_perspective,
                battles::created_at,
                battles::player_perspective_won,
            ))
            .load(conn)
            .expect("Error loading battles");

        let total = battles_query_count
            .filter(
                battles::id.eq_any(
                    trainer_mon_stats::table
                        .group_by(trainer_mon_stats::battle_id)
                        .select(trainer_mon_stats::battle_id)
                        .having(
                            sum(trainer_mon_stats::murders)
                                .ge(query_args.min_kos)
                                .and(sum(trainer_mon_stats::murders).le(query_args.max_kos))
                                .and(
                                    sum(trainer_mon_stats::damage_dealt)
                                        .ge(query_args.min_damage_dealt)
                                        .and(
                                            sum(trainer_mon_stats::damage_dealt)
                                                .le(query_args.max_damage_dealt),
                                        ),
                                ),
                        ),
                ),
            )
            .count()
            .get_result::<i64>(conn)
            .unwrap();

        (total, battles)
    };

    let battles = battles
        .into_iter()
        .map(
            |(
                battle_id,
                series_id,
                duration_seconds,
                seed,
                player_perspective,
                opponent_perspective,
                completed_at,
                player_perspective_won,
            )| {
                let completed_at = DateTime::from_timestamp(completed_at / 1000, 0).unwrap();
                let completed_at = completed_at.format("%Y-%m-%d %H:%M:%S").to_string() + " UTC";

                BattleSearchEntry {
                    battle_id,
                    series_id,
                    duration_seconds,
                    seed,
                    player_perspective,
                    opponent_perspective,
                    player_perspective_won,
                    completed_at,
                }
            },
        )
        .collect::<Vec<_>>();

    web::Json(PagedResponse::new(battles, page.limit, page.offset, total))
}

pub(crate) fn scope() -> actix_web::Scope {
    web::scope("/battle")
        .service(search)
        .service(video)
        .service(stats)
}

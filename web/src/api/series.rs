use actix_web::{get, web, Responder, Scope};
use all_battle_core::{gen3::GEN3, models, schema};
use diesel::dsl::{count, sum};
use diesel::prelude::*;
use log::{error, info};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::api::trainer::TrainerFilter;

use crate::{AppState, BattleResponse, MoveStats, SingleMonStats};

#[derive(Serialize)]
struct SeriesResponse {
    pub id: i32,
    pub tournament_id: i32,
    pub first_to: i32,
    pub priority: i32,
    pub completed_at: Option<i64>,
    pub trainers: Vec<i32>,
    pub battles: Vec<BattleResponse>,
}

#[get("/{id}")]
async fn get_series(data: web::Data<AppState>, series: web::Path<i32>) -> impl Responder {
    use schema::battles;
    use schema::series;
    use schema::series_trainers;
    let series_id = series.into_inner();

    let response = {
        let conn = &mut data.conn_pool.get().unwrap();

        let series: models::Series =
            match series::table.find(series_id).first::<models::Series>(conn) {
                Ok(series) => series,
                Err(err) => {
                    info!("Error loading series: {:?}", err);
                    return Err(actix_web::error::ErrorNotFound("Series not found"));
                }
            };

        let trainers: Vec<i32> = series_trainers::table
            .select(series_trainers::trainer_id)
            .filter(series_trainers::series_id.eq(series_id))
            .load::<i32>(conn)
            .expect("Error loading trainers");

        let battles: Vec<models::Battle> = battles::table
            .filter(schema::battles::series_id.eq(series_id))
            .load::<models::Battle>(conn)
            .expect("Error loading battles");

        SeriesResponse {
            id: series.id,
            tournament_id: series.tournament_id,
            priority: series.priority,
            first_to: series.first_to,
            completed_at: series.completed_at,
            trainers,
            battles: battles
                .into_iter()
                .map(|battle| BattleResponse {
                    id: battle.id,
                    player: battle.player_perspective,
                    opponent: battle.opponent_perspective,
                    player_won: battle.player_perspective_won,
                    seed: battle.seed,
                    video_path: battle.video_path,
                    duration: battle.duration,
                    events: battle
                        .events
                        .split(":")
                        .map(|s| s.to_string())
                        .filter(|s| !s.is_empty())
                        .collect(),
                    created_at: battle.created_at,
                })
                .collect(),
        }
    };

    Ok(web::Json(response))
}

fn default_mon_stats(trainer_id: i32) -> Vec<SingleMonStats> {
    GEN3.get_trainer_by_id(trainer_id)
        .party
        .iter()
        .map(|mon| SingleMonStats {
            mon_id: 1,
            battles: 0,
            damage_dealt: 0,
            damage_taken: 0,
            times_released: 0,
            murders: 0,
            deaths: 0,
            moves: GEN3
                .get_moves_for_trainer_mon(mon)
                .iter()
                .map(|m| MoveStats {
                    move_id: Some(m.id),
                    times_used: 0,
                    damage_dealt: 0,
                    murders: 0,
                })
                .collect(),
        })
        .collect()
}

#[derive(Serialize)]
struct TrainerSeriesStats {
    pub id: i32,
    pub mons: Vec<SingleMonStats>,
}

#[derive(Serialize)]
struct SeriesStats {
    pub stats: Vec<TrainerSeriesStats>,
}

#[derive(Queryable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerMonStatFoo {
    pub id: i32,
    pub trainer_id: i32,
    pub battle_id: i32,
    pub mon_index: i32,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub times_released: i32,
    pub murders: i32,
    pub deaths: i32,
}

#[get("/stats/{id}")]
async fn get_stats(data: web::Data<AppState>, series: web::Path<i32>) -> impl Responder {
    use schema::battles;
    use schema::series_trainers;
    use schema::trainer_mon_stats;
    use schema::trainer_mon_stats_moves;
    use schema::trainer_mons;

    let series_id = series.into_inner();

    let (trainers, mon_stats, move_stats) = {
        let conn = &mut data.conn_pool.get().unwrap();

        let trainers: Vec<i32> = series_trainers::table
            .select(series_trainers::trainer_id)
            .filter(series_trainers::series_id.eq(series_id))
            .load::<i32>(conn)
            .expect("Error loading trainers");

        let battles: Vec<i32> = battles::table
            .select(battles::id)
            .filter(battles::series_id.eq(series_id))
            .load::<i32>(conn)
            .expect("Error loading battles");

        let mon_stats: Vec<_> = trainer_mon_stats::table
            .inner_join(
                trainer_mons::table.on(trainer_mon_stats::trainer_mon_id.eq(trainer_mons::id)),
            )
            .filter(trainer_mon_stats::battle_id.eq_any(&battles))
            .select((
                trainer_mon_stats::id,
                trainer_mon_stats::trainer_id,
                trainer_mon_stats::battle_id,
                trainer_mons::party_index,
                trainer_mon_stats::damage_dealt,
                trainer_mon_stats::damage_taken,
                trainer_mon_stats::times_released,
                trainer_mon_stats::murders,
                trainer_mon_stats::deaths,
            ))
            .load::<TrainerMonStatFoo>(conn)
            .expect("Error loading mon stats");

        let move_stats: Vec<models::TrainerMonStatMove> = trainer_mon_stats_moves::table
            .select((
                trainer_mon_stats_moves::id,
                trainer_mon_stats_moves::trainer_mon_stats_id,
                trainer_mon_stats_moves::trainer_mon_target_id,
                trainer_mon_stats_moves::move_id,
                trainer_mon_stats_moves::times_used,
                trainer_mon_stats_moves::damage_dealt,
                trainer_mon_stats_moves::murders,
            ))
            .inner_join(
                trainer_mon_stats::table
                    .on(trainer_mon_stats::id.eq(trainer_mon_stats_moves::trainer_mon_stats_id)),
            )
            .filter(trainer_mon_stats::battle_id.eq_any(&battles))
            .load::<models::TrainerMonStatMove>(conn)
            .expect("Error loading move stats");

        (trainers, mon_stats, move_stats)
    };

    let mut stats = HashMap::new();
    for trainer in trainers {
        stats.insert(
            trainer,
            TrainerSeriesStats {
                id: trainer,
                mons: default_mon_stats(trainer),
            },
        );
    }
    let mut seen_battles = HashSet::new();

    for mon_stats in mon_stats {
        let trainer = stats.get_mut(&mon_stats.trainer_id).unwrap();
        let mon = trainer.mons.get_mut(mon_stats.mon_index as usize).unwrap();

        let seen_key = format!("{}-{}", mon_stats.trainer_id, mon_stats.battle_id);
        if !seen_battles.contains(&seen_key) {
            mon.battles += 1;
            seen_battles.insert(seen_key);
        }

        mon.damage_dealt += mon_stats.damage_dealt as i64;
        mon.damage_taken += mon_stats.damage_taken as i64;
        mon.times_released += mon_stats.times_released as i64;
        mon.murders += mon_stats.murders as i64;
        mon.deaths += mon_stats.deaths as i64;

        for move_stats in &move_stats {
            if move_stats.trainer_mon_stats_id == mon_stats.id {
                let mon_move = mon
                    .moves
                    .iter_mut()
                    .find(|m| m.move_id == Some(move_stats.move_id))
                    .unwrap();
                mon_move.times_used += move_stats.times_used as i64;
                mon_move.damage_dealt += move_stats.damage_dealt as i64;
                mon_move.murders += move_stats.murders as i64;
            }
        }
    }

    web::Json(SeriesStats {
        stats: stats.into_iter().map(|(_, v)| v).collect(),
    })
}

#[derive(Deserialize)]
struct SeriesSearchQuery {
    // A
    pub a_trainer_class: Option<String>,
    pub a_trainer_id: Option<i32>,
    pub a_number_of_mons: Option<i32>,
    pub a_mons: Option<String>,
    // B
    pub b_trainer_class: Option<String>,
    pub b_trainer_id: Option<i32>,
    pub b_number_of_mons: Option<i32>,
    pub b_mons: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct SeriesSearchBattleEntry {
    pub id: i32,
    pub winning_trainer_id: i32,
    pub duration: i32,
}

#[derive(Serialize)]
pub(crate) struct SeriesSearchEntry {
    pub series_id: i32,
    pub trainers: Vec<i32>,
    pub battle: Vec<SeriesSearchBattleEntry>,
}

#[derive(Serialize)]
pub(crate) struct SeriesSearchResponse {
    pub group_a_wins: i32,
    pub group_a: Vec<i32>,
    pub group_b_wins: i32,
    pub group_b: Vec<i32>,
    pub series: Vec<SeriesSearchEntry>,
}

#[get("/search")]
async fn get_search(
    data: web::Data<AppState>,
    query: web::Query<SeriesSearchQuery>,
) -> impl Responder {
    use schema::battles;
    use schema::series;
    use schema::series_trainers;

    let query = query.into_inner();

    let group_a = TrainerFilter::new(
        query.a_trainer_class.clone(),
        query.a_trainer_id,
        query.a_number_of_mons,
        query.a_mons.clone(),
    )
    .match_all();
    let group_b = TrainerFilter::new(
        query.b_trainer_class.clone(),
        query.b_trainer_id,
        query.b_number_of_mons,
        query.b_mons.clone(),
    )
    .match_all();
    let group_b: HashSet<i32> = group_b.difference(&group_a).cloned().collect();

    if group_a == group_b || group_a.len() == 0 || group_b.len() == 0 {
        return Err(actix_web::error::ErrorBadRequest("groups match"));
    }

    let matched_series = {
        let conn = &mut data.conn_pool.get().unwrap();

        let group_a_query = series_trainers::table
            .select(series_trainers::series_id)
            .filter(series_trainers::trainer_id.eq_any(&group_a));

        let group_b_query = series_trainers::table
            .select(series_trainers::series_id)
            .filter(series_trainers::trainer_id.eq_any(&group_b));

        let matched_series: Vec<i32> = series::table
            .select(series::id)
            .filter(series::completed_at.is_not_null())
            .filter(series::id.eq_any(group_a_query))
            .filter(series::id.eq_any(group_b_query))
            .load(conn)
            .expect("Error loading series");

        matched_series
    };

    let battles = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<models::Battle> = battles::table
            .filter(battles::series_id.eq_any(&matched_series))
            .load::<models::Battle>(conn)
            .expect("Error loading battles");

        battles
    };

    let mut series = HashMap::new();
    let mut group_a_wins = 0;
    let mut group_b_wins = 0;
    for battle in &battles {
        let entry = series.entry(battle.series_id).or_insert(SeriesSearchEntry {
            series_id: battle.series_id,
            trainers: {
                let mut trainers = vec![battle.player_perspective, battle.opponent_perspective];
                trainers.sort();
                trainers
            },
            battle: vec![],
        });

        entry.battle.push(SeriesSearchBattleEntry {
            id: battle.id,
            winning_trainer_id: if battle.player_perspective_won {
                battle.player_perspective
            } else {
                battle.opponent_perspective
            },
            duration: battle.duration,
        });

        if group_a.contains(&battle.player_perspective) {
            if battle.player_perspective_won {
                group_a_wins += 1;
            } else {
                group_b_wins += 1;
            }
        } else if group_b.contains(&battle.player_perspective) {
            if battle.player_perspective_won {
                group_b_wins += 1;
            } else {
                group_a_wins += 1;
            }
        } else {
            error!("Battle with unknown trainer: {}", battle.player_perspective);
        }
    }

    let mut series: Vec<SeriesSearchEntry> = series.into_iter().map(|(_, v)| v).collect();
    series.sort_by(|a, b| a.series_id.cmp(&b.series_id));

    Ok(web::Json(SeriesSearchResponse {
        group_a_wins,
        group_a: group_a.into_iter().collect(),
        group_b_wins,
        group_b: group_b.into_iter().collect(),
        series,
    }))
}

#[derive(Serialize)]
struct CompletedSeries {
    pub completed_count: i64,
    pub total_ids: i64,
    pub total_battles: i64,
    pub total_battle_duration: i64,
    pub latest_series_id: i32,
    pub start_timestamp: i64,
    pub last_timestamp: i64,
}

#[get("/completed")]
async fn get_completed(data: web::Data<AppState>) -> impl Responder {
    use schema::battles;
    use schema::series;
    let tournament_id = 1;

    let response = {
        let conn = &mut data.conn_pool.get().unwrap();

        let completed_count: i64 = series::table
            .filter(series::completed_at.is_not_null())
            .filter(series::tournament_id.eq(tournament_id))
            .select(count(series::id))
            .get_result::<i64>(conn)
            .expect("Error loading series");

        let series_count: i64 = series::table
            .filter(series::tournament_id.eq(tournament_id))
            .select(count(series::id))
            .count()
            .get_result::<i64>(conn)
            .expect("Error loading series count");

        let battles = battles::table
            .count()
            .get_result::<i64>(conn)
            .expect("Error loading battles");

        let total_battle_duration = battles::table
            .select(sum(battles::duration))
            .get_result::<Option<i64>>(conn)
            .expect("Error loading total battle duration")
            .unwrap_or(0)
            * 1000;

        let series: i32 = series::table
            .order(series::completed_at.desc())
            .select(series::id)
            .first(conn)
            .expect("Error loading series");

        let earliest_series_timestamp: Option<i64> = series::table
            .filter(series::completed_at.is_not_null())
            .order(series::completed_at.asc())
            .select(series::completed_at)
            .first(conn)
            .expect("Error loading earliest series");

        let latest_series_timestamp: Option<i64> = series::table
            .filter(series::completed_at.is_not_null())
            .order(series::completed_at.desc())
            .select(series::completed_at)
            .first(conn)
            .expect("Error loading latest series");

        CompletedSeries {
            completed_count,
            total_ids: series_count,
            total_battles: battles,
            total_battle_duration,
            latest_series_id: series,
            start_timestamp: earliest_series_timestamp.unwrap_or(0),
            last_timestamp: latest_series_timestamp.unwrap_or(0),
        }
    };

    web::Json(response)
}

pub(crate) fn scope() -> Scope {
    web::scope("/series")
        .service(get_stats)
        .service(get_search)
        .service(get_completed)
        .service(get_series)
}

use actix_web::{get, web, Responder, Scope};
use all_battle_core::{gen3::GEN3, models, schema};
use diesel::prelude::*;
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::api::series::{SeriesSearchBattleEntry, SeriesSearchEntry};
use crate::api::TOURNAMENT_ID;
use crate::progress::{get_progress_of_trainer_id, RouteProgressSet};
use crate::{AppState, SingleMonStats};

#[derive(Debug)]
pub(crate) struct TrainerFilter {
    pub trainer_class: Option<String>,
    pub trainer_id: Option<i32>,
    pub number_of_mons: Option<i32>,
    pub mons: Vec<String>,
}

impl TrainerFilter {
    pub fn new(
        trainer_class: Option<String>,
        trainer_id: Option<i32>,
        number_of_mons: Option<i32>,
        mons: Option<String>,
    ) -> Self {
        let mons: Vec<String> = if let Some(mons) = &mons {
            mons.split(",")
                .filter(|i| i.len() > 0)
                .map(|i| i.to_string())
                .collect()
        } else {
            vec![]
        };

        TrainerFilter {
            trainer_class,
            trainer_id,
            number_of_mons,
            mons,
        }
    }

    pub fn match_all(&self) -> HashSet<i32> {
        let mut filtered_trainers = HashSet::new();
        if let Some(trainer_id) = self.trainer_id {
            filtered_trainers.insert(trainer_id);
        } else {
            for trainer in &GEN3.trainers {
                if let Some(trainer_class) = &self.trainer_class {
                    if trainer_class != &trainer.trainer_class {
                        continue;
                    }
                }

                if let Some(number_of_mons) = self.number_of_mons {
                    if number_of_mons != trainer.party_size as i32 {
                        continue;
                    }
                }

                if self.mons.len() > 0 {
                    let mut found = false;
                    for mon in &self.mons {
                        if trainer.party.iter().any(|m| m.species == *mon) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        continue;
                    }
                }

                filtered_trainers.insert(trainer.id);
            }
        }

        filtered_trainers
    }
}

#[derive(Serialize, Clone)]
struct TrainerRankBrief {
    pub id: i32,
    pub wins: i64,
    pub total: i64,
    pub overall_rank: i32,
}

#[derive(Serialize, Clone)]
struct TrainersRanked {
    pub trainers: Vec<TrainerRankBrief>,
}

#[derive(Deserialize)]
struct TrainersRankQuery {
    #[serde(default)]
    pub trainer_class: Option<String>,
    #[serde(default)]
    pub trainer_id: Option<i32>,
    #[serde(default)]
    pub number_of_mons: Option<i32>,
    #[serde(default)]
    pub mons: Option<String>,
}

#[get("/rank")]
async fn get_rank(
    data: web::Data<AppState>,
    query: web::Query<TrainersRankQuery>,
) -> impl Responder {
    use schema::battles;
    use schema::series;

    let query = query.into_inner();

    let filter = TrainerFilter::new(
        query.trainer_class.clone(),
        query.trainer_id,
        query.number_of_mons,
        query.mons.clone(),
    );

    let matched_trainers = filter.match_all();

    let battles = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<(i32, i32, bool)> = battles::table
            .select((
                battles::player_perspective,
                battles::opponent_perspective,
                battles::player_perspective_won,
            ))
            .inner_join(series::table.on(series::id.eq(battles::series_id)))
            .filter(battles::player_perspective.eq_any(&matched_trainers))
            .filter(battles::opponent_perspective.eq_any(&matched_trainers))
            .filter(series::tournament_id.eq(TOURNAMENT_ID))
            .load(conn)
            .expect("Error loading total");

        battles
    };

    let trainers = matched_trainers.into_iter().collect::<Vec<i32>>();

    let mut trainers = {
        let mut result = HashMap::new();
        for player in trainers {
            if !result.contains_key(&player) {
                result.insert(
                    player,
                    TrainerRankBrief {
                        id: player,
                        overall_rank: -1,
                        wins: 0,
                        total: 0,
                    },
                );
            }
        }

        result
    };

    for (player_perspective, opponent_perspective, player_perspective_won) in battles {
        if let Some(player) = trainers.get_mut(&player_perspective) {
            player.total += 1;

            if player_perspective_won {
                player.wins += 1;
            }
        }

        if let Some(opponent) = trainers.get_mut(&opponent_perspective) {
            opponent.total += 1;

            if !player_perspective_won {
                opponent.wins += 1;
            }
        }
    }

    let mut trainers: Vec<TrainerRankBrief> = trainers.into_iter().map(|(_, v)| v).collect();
    trainers.sort_by(|a, b| {
        let a_percent = a.wins as f64 / a.total as f64;
        let b_percent = b.wins as f64 / b.total as f64;
        let cmp = a_percent
            .partial_cmp(&b_percent)
            .unwrap_or(std::cmp::Ordering::Equal);
        if cmp == std::cmp::Ordering::Equal {
            let cmp = a.wins.cmp(&b.wins);
            if cmp == std::cmp::Ordering::Equal {
                let cmp = a.total.cmp(&b.total);
                if cmp == std::cmp::Ordering::Equal {
                    return a.id.cmp(&b.id);
                }
                return cmp.reverse();
            }
            return cmp.reverse();
        }
        cmp.reverse()
    });

    for (i, trainer) in trainers.iter_mut().enumerate() {
        trainer.overall_rank = i as i32;
    }

    web::Json(TrainersRanked { trainers })
}

#[derive(Serialize, Clone)]
struct TrainerRank {
    pub id: i32,
    pub wins: i64,
    pub total: i64,
    pub overall_rank: i32,
    pub mons: Vec<SingleMonStats>,
}

#[derive(Serialize)]
struct TrainerStats {
    pub rank: TrainerRank,
    pub series: Vec<SeriesSearchEntry>,
}

#[derive(Deserialize)]
struct TrainerStatsQuery {
    #[serde(default)]
    exclude_after: Option<i64>,
    #[serde(default)]
    pub trainer_class: Option<String>,
    #[serde(default)]
    pub trainer_id: Option<i32>,
    #[serde(default)]
    pub number_of_mons: Option<i32>,
    #[serde(default)]
    pub mons: Option<String>,
}

#[get("/stats/{id}")]
async fn get_stats(
    data: web::Data<AppState>,
    trainer_id: web::Path<i32>,
    query: web::Query<TrainerStatsQuery>,
) -> impl Responder {
    use schema::battles;

    let query = query.into_inner();

    let mut now = chrono::Utc::now().timestamp_millis();

    let exclude_after = query.exclude_after.unwrap_or(std::i64::MAX);

    let filter = TrainerFilter::new(
        query.trainer_class.clone(),
        query.trainer_id,
        query.number_of_mons,
        query.mons.clone(),
    );

    let trainers = filter.match_all();
    debug!(
        "Got trainer filter {}",
        chrono::Utc::now().timestamp_millis() - now
    );
    now = chrono::Utc::now().timestamp_millis();

    let trainer_id = trainer_id.into_inner();

    let (battles, as_player, as_opponent) = {
        let conn = &mut data.conn_pool.get().unwrap();

        let as_player_wins = battles::table
            .filter(battles::player_perspective.eq(trainer_id))
            .filter(battles::player_perspective_won.eq(true))
            .filter(battles::created_at.lt(exclude_after))
            .filter(battles::player_perspective.eq_any(&trainers))
            .filter(battles::opponent_perspective.eq_any(&trainers))
            .count()
            .get_result::<i64>(conn)
            .expect("Error loading total");

        let as_opponent_wins = battles::table
            .filter(battles::opponent_perspective.eq(trainer_id))
            .filter(battles::player_perspective_won.eq(false))
            .filter(battles::created_at.lt(exclude_after))
            .filter(battles::player_perspective.eq_any(&trainers))
            .filter(battles::opponent_perspective.eq_any(&trainers))
            .count()
            .get_result::<i64>(conn)
            .expect("Error loading total");

        let battles_total = battles::table
            .filter(battles::opponent_perspective.eq(trainer_id).or(battles::player_perspective.eq(trainer_id)))
            .filter(battles::created_at.lt(exclude_after))
            .filter(battles::player_perspective.eq_any(&trainers))
            .filter(battles::opponent_perspective.eq_any(&trainers))
            .count()
            .get_result::<i64>(conn)
            .expect("Error loading total");

        (
            battles_total,
            as_player_wins,
            as_opponent_wins,
        )
    };
    debug!(
        "Got trainer counts {}",
        chrono::Utc::now().timestamp_millis() - now
    );
    now = chrono::Utc::now().timestamp_millis();

    let ranking =
        data.trainer_rank_cache
            .get(&mut data.conn_pool.get().unwrap(), exclude_after, &trainers);
    let rank = ranking
        .iter()
        .position(|r| r.id == trainer_id)
        .map(|i| i as i32)
        .unwrap();
    debug!(
        "Got trainer rank {}",
        chrono::Utc::now().timestamp_millis() - now
    );
    now = chrono::Utc::now().timestamp_millis();

    let series = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<models::Battle> = battles::table
            .filter(
                battles::player_perspective
                    .eq(trainer_id)
                    .or(battles::opponent_perspective.eq(trainer_id)),
            )
            .filter(battles::player_perspective.eq_any(&trainers))
            .filter(battles::opponent_perspective.eq_any(&trainers))
            .filter(battles::created_at.lt(exclude_after))
            .load::<models::Battle>(conn)
            .expect("Error loading series");

        let mut series = HashMap::new();
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
        }

        let mut series: Vec<SeriesSearchEntry> = series.into_iter().map(|(_, v)| v).collect();
        series.sort_by(|a, b| a.series_id.cmp(&b.series_id));
        series
    };
    debug!(
        "Got trainer series {}",
        chrono::Utc::now().timestamp_millis() - now
    );
    now = chrono::Utc::now().timestamp_millis();

    let mons = data.trainer_mon_cache.get(
        &mut data.conn_pool.get().unwrap(),
        trainer_id,
        exclude_after,
        &trainers,
    );
    debug!(
        "Got trainer mons {}",
        chrono::Utc::now().timestamp_millis() - now
    );

    web::Json(TrainerStats {
        rank: TrainerRank {
            id: trainer_id,
            wins: as_player + as_opponent,
            overall_rank: rank,
            total: battles,
            mons,
        },
        series,
    })
}

#[derive(Serialize)]
struct TrainerHistoryBattle {
    pub series_id: i32,
    pub opponent: i32,
    pub won: bool,
}

#[derive(Serialize)]
struct TrainerHistory {
    pub id: i32,
    pub wins: i64,
    pub total: i64,
    pub history: Vec<TrainerHistoryBattle>,
}

#[get("/history/{id}")]
async fn get_history(data: web::Data<AppState>, trainer: web::Path<i32>) -> impl Responder {
    use schema::battles;

    let trainer_id = trainer.into_inner();

    let battles = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<models::Battle> = match battles::table
            .filter(
                battles::player_perspective
                    .eq(trainer_id)
                    .or(battles::opponent_perspective.eq(trainer_id)),
            )
            .load::<models::Battle>(conn)
        {
            Ok(battles) => battles,
            Err(_) => vec![],
        };

        battles
    };

    let mut wins = 0;
    let mut total = 0;
    let mut history = vec![];

    for battle in &battles {
        let (opponent, won) = if battle.player_perspective == trainer_id {
            (battle.opponent_perspective, battle.player_perspective_won)
        } else {
            (battle.player_perspective, !battle.player_perspective_won)
        };

        if won {
            wins += 1;
        }
        total += 1;

        history.push(TrainerHistoryBattle {
            series_id: battle.series_id,
            opponent,
            won,
        });
    }

    web::Json(TrainerHistory {
        id: trainer_id,
        wins,
        total,
        history,
    })
}

#[derive(Serialize)]
struct TrainerProgressResponse {
    pub id: i32,
    pub progress: RouteProgressSet,
}

#[get("/progress/{id}")]
async fn get_game_progress(data: web::Data<AppState>, trainer: web::Path<i32>) -> impl Responder {
    let trainer = trainer.into_inner();

    let conn = &mut data.conn_pool.get().unwrap();

    let progress_set = get_progress_of_trainer_id(conn, trainer);

    web::Json(TrainerProgressResponse {
        id: trainer,
        progress: progress_set,
    })
}

pub(crate) fn scope() -> Scope {
    web::scope("/trainer")
        .service(get_rank)
        .service(get_stats)
        .service(get_history)
        .service(get_game_progress)
}

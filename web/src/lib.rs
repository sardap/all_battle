#![feature(duration_constructors)]
use crate::api::series::SeriesSearchBattleEntry;
use actix_web::{web, Scope};
use all_battle_core::{gen3::GEN3, models, schema};
use api::series::SeriesSearchEntry;
use diesel::dsl::{count, sum};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;
use log::debug;
use moka::sync::Cache;
use serde_derive::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

mod api;
mod frontend;
mod progress;

fn get_mon_stats(conn: &mut PooledConnection, mon_id: i32) -> Option<SingleMonStats> {
    use schema::battles;
    use schema::trainer_mon_stats;
    use schema::trainer_mon_stats_moves;

    let (battles, damage_dealt, damage_taken, times_released, murders, deaths) = {
        let (battles, damage_dealt, damage_taken, times_released, murders, deaths): (
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
        ) = match trainer_mon_stats::table
            .filter(trainer_mon_stats::mon_id.eq(mon_id))
            .select((
                count(trainer_mon_stats::id),
                sum(trainer_mon_stats::damage_dealt),
                sum(trainer_mon_stats::damage_taken),
                sum(trainer_mon_stats::times_released),
                sum(trainer_mon_stats::murders),
                sum(trainer_mon_stats::deaths),
            ))
            .inner_join(battles::table.on(battles::id.eq(trainer_mon_stats::battle_id)))
            .first(conn)
        {
            Ok((battles, damage_dealt, damage_taken, times_released, murders, deaths)) => (
                battles,
                damage_dealt,
                damage_taken,
                times_released,
                murders,
                deaths,
            ),
            Err(_) => return None,
        };

        (
            battles,
            damage_dealt,
            damage_taken,
            times_released,
            murders,
            deaths,
        )
    };

    let moves = {
        let moves: Vec<(i32, Option<i64>, Option<i64>, Option<i64>)> =
            trainer_mon_stats_moves::table
                .group_by(trainer_mon_stats_moves::move_id)
                .select((
                    trainer_mon_stats_moves::move_id,
                    sum(trainer_mon_stats_moves::times_used),
                    sum(trainer_mon_stats_moves::damage_dealt),
                    sum(trainer_mon_stats_moves::murders),
                ))
                .inner_join(
                    trainer_mon_stats::table.on(trainer_mon_stats::id
                        .eq(trainer_mon_stats_moves::trainer_mon_stats_id)),
                )
                .inner_join(battles::table.on(battles::id.eq(trainer_mon_stats::battle_id)))
                .filter(trainer_mon_stats::mon_id.eq(mon_id))
                .order_by(trainer_mon_stats_moves::move_id)
                .load(conn)
                .expect("Error loading total");

        moves
    };

    let result = SingleMonStats {
        mon_id,
        battles,
        damage_dealt: damage_dealt.unwrap_or_default(),
        damage_taken: damage_taken.unwrap_or_default(),
        times_released: times_released.unwrap_or_default(),
        murders: murders.unwrap_or_default(),
        deaths: deaths.unwrap_or_default(),
        moves: moves
            .into_iter()
            .map(|(move_id, times_used, damage_dealt, murders)| MoveStats {
                move_id: Some(move_id),
                times_used: times_used.unwrap_or_default(),
                damage_dealt: damage_dealt.unwrap_or_default(),
                murders: murders.unwrap_or_default(),
            })
            .collect(),
    };

    Some(result)
}

#[derive(Serialize, Clone)]
struct MoveStats {
    move_id: Option<i32>,
    times_used: i64,
    damage_dealt: i64,
    murders: i64,
}

#[derive(Serialize, Default, Clone)]
struct SingleMonStats {
    mon_id: i32,
    battles: i64,
    damage_dealt: i64,
    damage_taken: i64,
    times_released: i64,
    murders: i64,
    deaths: i64,
    moves: Vec<MoveStats>,
}

fn trainer_mon_stats(
    conn: &mut PooledConnection,
    trainer_id: i32,
    exclude_after: i64,
    trainers: &HashSet<i32>,
) -> Vec<SingleMonStats> {
    use schema::battles;
    use schema::trainer_mon_stats;
    use schema::trainer_mon_stats_moves;
    use schema::trainer_mons;

    let mut now = chrono::Utc::now().timestamp_millis();

    let mon_stats = {
        let mon_stats: Vec<(
            i32,
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
        )> = trainer_mon_stats::table
            .inner_join(
                trainer_mons::table.on(trainer_mons::id.eq(trainer_mon_stats::trainer_mon_id)),
            )
            .group_by(trainer_mons::id)
            .select((
                trainer_mons::party_index,
                count(trainer_mon_stats::id),
                sum(trainer_mon_stats::damage_dealt),
                sum(trainer_mon_stats::damage_taken),
                sum(trainer_mon_stats::times_released),
                sum(trainer_mon_stats::murders),
                sum(trainer_mon_stats::deaths),
            ))
            .inner_join(battles::table.on(battles::id.eq(trainer_mon_stats::battle_id)))
            .filter(
                trainer_mon_stats::trainer_id
                    .eq(trainer_id)
                    .and(battles::created_at.lt(exclude_after))
                    .and(battles::player_perspective.eq_any(trainers))
                    .and(battles::opponent_perspective.eq_any(trainers)),
            )
            .order_by(trainer_mons::party_index)
            .load(conn)
            .expect("Error loading total");

        mon_stats
    };
    debug!(
        "Mon stats took vec {}ms",
        chrono::Utc::now().timestamp_millis() - now
    );

    let trainer = GEN3.get_trainer_by_id(trainer_id);

    let mut result = vec![];
    for mon_index in 0..trainer.party.len() {
        now = chrono::Utc::now().timestamp_millis();
        let mon_moves = GEN3.get_moves_for_trainer_mon(&trainer.party[mon_index]);

        if let Some((
            mon_index,
            battles,
            damage_dealt,
            damage_taken,
            times_released,
            murders,
            deaths,
        )) = mon_stats
            .iter()
            .find(|(i, _, _, _, _, _, _)| *i == mon_index as i32)
        {
            let move_stats = {
                let moves: Vec<(i32, Option<i64>, Option<i64>, Option<i64>)> =
                    trainer_mon_stats_moves::table
                        .group_by(trainer_mon_stats_moves::move_id)
                        .select((
                            trainer_mon_stats_moves::move_id,
                            sum(trainer_mon_stats_moves::times_used),
                            sum(trainer_mon_stats_moves::damage_dealt),
                            sum(trainer_mon_stats_moves::murders),
                        ))
                        .inner_join(trainer_mon_stats::table.on(
                            trainer_mon_stats::id.eq(trainer_mon_stats_moves::trainer_mon_stats_id),
                        ))
                        .inner_join(battles::table.on(battles::id.eq(trainer_mon_stats::battle_id)))
                        .inner_join(
                            trainer_mons::table
                                .on(trainer_mons::id.eq(trainer_mon_stats::trainer_mon_id)),
                        )
                        .filter(
                            trainer_mon_stats::trainer_id
                                .eq(trainer_id)
                                .and(trainer_mons::party_index.eq(mon_index))
                                .and(battles::created_at.lt(exclude_after))
                                .and(battles::player_perspective.eq_any(trainers))
                                .and(battles::opponent_perspective.eq_any(trainers)),
                        )
                        .order_by(trainer_mon_stats_moves::move_id)
                        .load(conn)
                        .expect("Error loading total");

                moves
            };

            let move_stats = {
                let mut result = vec![];
                for mon_move in mon_moves.into_iter() {
                    if let Some((_, times_used, damage_dealt, murders)) = move_stats
                        .iter()
                        .find(|(move_index, _, _, _)| *move_index == mon_move.id)
                    {
                        result.push(MoveStats {
                            move_id: Some(mon_move.id),
                            times_used: times_used.unwrap_or_default(),
                            damage_dealt: damage_dealt.unwrap_or_default(),
                            murders: murders.unwrap_or_default(),
                        });
                    } else {
                        result.push(MoveStats {
                            move_id: Some(mon_move.id),
                            times_used: 0,
                            damage_dealt: 0,
                            murders: 0,
                        });
                    }
                }
                result
            };

            result.push(SingleMonStats {
                mon_id: GEN3
                    .get_mon_by_species(&trainer.party[*mon_index as usize].species)
                    .id,
                battles: *battles,
                damage_dealt: damage_dealt.unwrap_or_default(),
                damage_taken: damage_taken.unwrap_or_default(),
                times_released: times_released.unwrap_or_default(),
                murders: murders.unwrap_or_default(),
                deaths: deaths.unwrap_or_default(),
                moves: move_stats,
            });
        } else {
            result.push(SingleMonStats {
                mon_id: 0,
                battles: 0,
                damage_dealt: 0,
                damage_taken: 0,
                times_released: 0,
                murders: 0,
                deaths: 0,
                moves: mon_moves
                    .into_iter()
                    .map(|m| MoveStats {
                        move_id: Some(m.id),
                        times_used: 0,
                        damage_dealt: 0,
                        murders: 0,
                    })
                    .collect(),
            });
        }
        debug!(
            "Single mon stats {}ms",
            chrono::Utc::now().timestamp_millis() - now
        );
    }

    result
}

#[derive(Serialize)]
struct BattleResponse {
    pub id: i32,
    pub player: i32,
    pub opponent: i32,
    pub player_won: bool,
    pub seed: i32,
    pub video_path: String,
    pub duration: i32,
    pub events: Vec<String>,
    pub created_at: i64,
}

fn convert_mon_stats(
    conn: &mut PooledConnection,
    mon_stats: Vec<models::TrainerMonStat>,
) -> Vec<SingleMonStats> {
    use schema::trainer_mon_stats_moves;

    mon_stats
        .into_iter()
        .map(|mon_stats| SingleMonStats {
            mon_id: mon_stats.mon_id,
            battles: mon_stats.times_released as i64,
            damage_dealt: mon_stats.damage_dealt as i64,
            damage_taken: mon_stats.damage_taken as i64,
            times_released: mon_stats.times_released as i64,
            murders: mon_stats.murders as i64,
            deaths: mon_stats.deaths as i64,
            moves: {
                let raw_move_stats = {
                    trainer_mon_stats_moves::table
                        .filter(trainer_mon_stats_moves::trainer_mon_stats_id.eq(mon_stats.id))
                        .load::<models::TrainerMonStatMove>(conn)
                        .expect("Error loading move stats")
                };

                raw_move_stats
                    .into_iter()
                    .map(|move_stats| MoveStats {
                        move_id: Some(move_stats.move_id),
                        times_used: move_stats.times_used as i64,
                        damage_dealt: move_stats.damage_dealt as i64,
                        murders: move_stats.murders as i64,
                    })
                    .collect()
            },
        })
        .collect()
}

#[derive(Clone)]
pub struct TrainerMonCache {
    cache: Arc<Mutex<Cache<String, Vec<SingleMonStats>>>>,
}

impl TrainerMonCache {
    pub fn new() -> Self {
        TrainerMonCache {
            cache: Arc::new(Mutex::new(
                Cache::builder()
                    .max_capacity(10_000)
                    .time_to_live(Duration::from_days(364635))
                    .build(),
            )),
        }
    }

    fn get(
        &self,
        conn: &mut PooledConnection,
        trainer_id: i32,
        exclude_after: i64,
        trainers: &HashSet<i32>,
    ) -> Vec<SingleMonStats> {
        let trainer_hash = {
            let mut trainers_sorted = trainers.iter().map(|i| *i).collect::<Vec<i32>>();
            trainers_sorted.sort();
            md5::compute(format!("{:?}", trainers_sorted)).0
        };

        let key = format!("{}-{}-{:X?}", exclude_after, trainer_id, trainer_hash);
        if let Some(stats) = self.cache.lock().unwrap().get(&key) {
            return stats.clone();
        }

        let stats = trainer_mon_stats(conn, trainer_id, exclude_after, trainers);
        self.cache.lock().unwrap().insert(key, stats.clone());
        stats
    }
}

#[derive(Serialize, Clone)]
struct TrainerBareRank {
    id: i32,
    wins: i64,
    total: i64,
}

fn get_complete_trainer_ranking(
    conn: &mut PooledConnection,
    exclude_after: i64,
    trainers: &HashSet<i32>,
) -> Vec<TrainerBareRank> {
    debug!("Getting battles {}", trainers.len());

    let battles = {
        let battles: Vec<(i32, i32, bool)> = schema::battles::table
            .select((
                schema::battles::player_perspective,
                schema::battles::opponent_perspective,
                schema::battles::player_perspective_won,
            ))
            .filter(schema::battles::created_at.lt(exclude_after))
            .filter(schema::battles::player_perspective.eq_any(trainers))
            .filter(schema::battles::opponent_perspective.eq_any(trainers))
            .load(conn)
            .expect("Error loading total");

        battles
    };

    debug!("Getting trainers");

    let trainers = {
        let trainers: Vec<i32> = schema::series_trainers::table
            .select(schema::series_trainers::trainer_id)
            .group_by(schema::series_trainers::trainer_id)
            .load(conn)
            .expect("Error loading total");

        trainers
    };

    debug!("Getting scores");

    let mut scores = HashMap::new();
    for player in trainers {
        if !scores.contains_key(&player) {
            scores.insert(
                player,
                TrainerBareRank {
                    id: player,
                    wins: 0,
                    total: 0,
                },
            );
        }
    }

    for (player_perspective, opponent_perspective, player_perspective_won) in battles {
        {
            let player = scores.get_mut(&player_perspective).unwrap();

            if player_perspective_won {
                player.wins += 1;
            }

            player.total += 1;
        }

        {
            let opponent = scores.get_mut(&opponent_perspective).unwrap();

            if !player_perspective_won {
                opponent.wins += 1;
            }

            opponent.total += 1;
        }
    }

    let mut scores: Vec<(i32, TrainerBareRank)> = scores.into_iter().collect();

    scores.sort_by(|(a_id, a_stats), (b_id, b_stats)| {
        let a_percent = a_stats.wins as f64 / a_stats.total as f64;
        let b_percent = b_stats.wins as f64 / b_stats.total as f64;
        let cmp = a_percent
            .partial_cmp(&b_percent)
            .unwrap_or(std::cmp::Ordering::Equal);
        if cmp == std::cmp::Ordering::Equal {
            let cmp = a_stats.wins.cmp(&b_stats.wins);
            if cmp == std::cmp::Ordering::Equal {
                let cmp = a_stats.total.cmp(&b_stats.total);
                if cmp == std::cmp::Ordering::Equal {
                    return a_id.cmp(b_id);
                }
                return cmp;
            }
            return cmp;
        }
        cmp
    });
    scores.reverse();

    debug!("Got scores");

    scores.into_iter().map(|i| i.1).collect()
}

#[derive(Clone)]
pub struct TrainerRankCache {
    cache: Arc<Mutex<Cache<[u8; 16], Vec<TrainerBareRank>>>>,
}

impl TrainerRankCache {
    pub fn new() -> Self {
        TrainerRankCache {
            cache: Arc::new(Mutex::new(
                Cache::builder()
                    .max_capacity(100)
                    .time_to_live(Duration::from_days(364635))
                    .build(),
            )),
        }
    }

    fn get(
        &self,
        conn: &mut PooledConnection,
        exclude_after: i64,
        trainers: &HashSet<i32>,
    ) -> Vec<TrainerBareRank> {
        let seconds = exclude_after / 1000;
        let exclude_after = seconds - (seconds % 300);
        let exclude_after = exclude_after * 1000;

        // Get md5 hash of the trainers
        let key = {
            let mut trainers_sorted = trainers.iter().map(|i| *i).collect::<Vec<i32>>();
            trainers_sorted.sort();
            md5::compute(format!("{:?}", trainers_sorted)).0
        };

        if let Some(stats) = self.cache.lock().unwrap().get(&key) {
            return stats.clone();
        }

        let stats = get_complete_trainer_ranking(conn, exclude_after, trainers);
        self.cache.lock().unwrap().insert(key, stats.clone());
        stats
    }
}

#[derive(Clone)]
pub struct MonStatsCache {
    cache: Arc<Mutex<Cache<i32, SingleMonStats>>>,
}

impl MonStatsCache {
    pub fn new() -> Self {
        MonStatsCache {
            cache: Arc::new(Mutex::new(
                Cache::builder()
                    .max_capacity(386)
                    .time_to_live(Duration::from_days(364635))
                    .build(),
            )),
        }
    }

    fn get(&self, conn: &mut PooledConnection, mon_id: i32) -> Option<SingleMonStats> {
        if let Some(stats) = self.cache.lock().unwrap().get(&mon_id) {
            return Some(stats.clone());
        }

        match get_mon_stats(conn, mon_id) {
            Some(stats) => {
                self.cache.lock().unwrap().insert(mon_id, stats.clone());
                Some(stats)
            }
            None => None,
        }
    }
}

pub struct AppState {
    conn_pool: ConnectionPool,
    trainer_mon_cache: TrainerMonCache,
    trainer_rank_cache: TrainerRankCache,
    mon_stats_cache: MonStatsCache,
    video_path_root: PathBuf,
    build_dir: PathBuf,
}

impl AppState {
    pub fn new(
        conn_pool: ConnectionPool,
        video_path_root: PathBuf,
        build_path: PathBuf,
        trainer_mon_cache: TrainerMonCache,
        trainer_rank_cache: TrainerRankCache,
        mon_stats_cache: MonStatsCache,
    ) -> Self {
        AppState {
            conn_pool,
            trainer_mon_cache,
            trainer_rank_cache,
            mon_stats_cache,
            video_path_root,
            build_dir: build_path,
        }
    }
}

type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool(url: &str) -> ConnectionPool {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn routes() -> Scope {
    web::scope("")
        .service(crate::api::scope())
        .service(crate::frontend::scope())
}

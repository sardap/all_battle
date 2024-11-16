use actix_web::{get, web, Responder, Scope};
use all_battle_core::{models, schema};
use diesel::dsl::{count, sum};
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{AppState, SeriesSearchBattleEntry, SeriesSearchEntry};

#[get("/stats/{mon_id}")]
async fn get_stats(data: web::Data<AppState>, mon_id: web::Path<i32>) -> impl Responder {
    match data
        .mon_stats_cache
        .get(&mut data.conn_pool.get().unwrap(), mon_id.into_inner())
    {
        Some(stats) => Ok(web::Json(stats)),
        None => Err(actix_web::error::ErrorNotFound("Mon stats not found")),
    }
}

#[derive(Serialize)]
struct MonSeriesResponse {
    owning_trainers: Vec<i32>,
    wins: f64,
    total: i64,
    series: Vec<SeriesSearchEntry>,
}

#[get("/series/{mon_id}")]
async fn get_series(data: web::Data<AppState>, mon_id: web::Path<i32>) -> impl Responder {
    use schema::battles;
    use schema::trainer_mon_stats;

    let mon_id = mon_id.into_inner();

    let battles = {
        let conn = &mut data.conn_pool.get().unwrap();

        let battles: Vec<models::Battle> = battles::table
            .filter(
                battles::id.eq_any(
                    trainer_mon_stats::table
                        .select(trainer_mon_stats::battle_id)
                        .filter(trainer_mon_stats::mon_id.eq(mon_id))
                        .filter(trainer_mon_stats::times_released.gt(0)),
                ),
            )
            .load::<models::Battle>(conn)
            .expect("Error loading battles");

        battles
    };

    let owning_trainers = {
        let conn = &mut data.conn_pool.get().unwrap();

        let trainers: Vec<i32> = trainer_mon_stats::table
            .filter(trainer_mon_stats::mon_id.eq(mon_id))
            .select(trainer_mon_stats::trainer_id)
            .distinct()
            .load::<i32>(conn)
            .expect("Error loading trainers");

        trainers
    };

    let owning_trainers: HashSet<i32> = owning_trainers.into_iter().collect();

    let total = battles.len() as i64;
    let mut wins = 0.0;

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

        if owning_trainers.contains(&battle.player_perspective)
            && owning_trainers.contains(&battle.opponent_perspective)
        {
            // Both sides own the mon ez win and loss lets just  call it .5 and be done
            wins += 0.5;
        } else if owning_trainers.contains(&battle.player_perspective) {
            // Player owns the mon
            if battle.player_perspective_won {
                wins += 1.;
            }
        } else if owning_trainers.contains(&battle.opponent_perspective) {
            // Opponent owns the mon
            if !battle.player_perspective_won {
                wins += 1.;
            }
        };

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

    web::Json(MonSeriesResponse {
        owning_trainers: owning_trainers.into_iter().collect(),
        wins,
        total,
        series,
    })
}

struct MonFilter {
    filter_type: Vec<String>,
    abilities: Vec<String>,
}

impl MonFilter {
    fn get_matching_mons(&self, conn: &mut SqliteConnection) -> HashSet<i32> {
        use schema::mon_abilities;
        use schema::mon_type_mapping;
        use schema::mons;

        let mut result = HashSet::new();

        let mut query = mons::table
            .select(mons::id)
            .inner_join(mon_type_mapping::table.on(mon_type_mapping::mon_id.eq(mons::id)))
            .inner_join(mon_abilities::table.on(mon_abilities::mon_id.eq(mons::id)))
            .into_boxed();

        if !self.filter_type.is_empty() {
            query = query.filter(mon_type_mapping::mon_type.eq_any(&self.filter_type));
        }

        if !self.abilities.is_empty() {
            query = query.filter(mon_abilities::ability.eq_any(&self.abilities));
        }

        let mons: Vec<i32> = query.load(conn).expect("Error loading mons");

        for mon in mons {
            result.insert(mon);
        }

        result
    }
}

#[derive(Serialize)]
struct MonRanking {
    mon_id: i32,
    murders: i64,
    deaths: i64,
    times_released: i64,
    damage_dealt: i64,
    damage_taken: i64,
    average_level: f64,
    number_exist: i64,
}

#[derive(Serialize)]
struct MonRankResponse {
    mons: Vec<MonRanking>,
}

#[derive(Deserialize)]
struct MonRankQuery {
    types: Option<String>,
    abilities: Option<String>,
}

impl MonRankQuery {
    fn get_filter(&self) -> MonFilter {
        let types = match &self.types {
            Some(s) => s.split(',').map(|s| s.to_string()).collect(),
            None => vec![],
        };
        let abilities = match &self.abilities {
            Some(s) => s.split(',').map(|s| s.to_string()).collect(),
            None => vec![],
        };

        MonFilter {
            filter_type: types,
            abilities,
        }
    }
}

#[get("/rank")]
async fn get_rank(data: web::Data<AppState>, query: web::Query<MonRankQuery>) -> impl Responder {
    let query = query.into_inner();
    let filter = query.get_filter();

    let mon_ids: HashSet<i32>;
    {
        let conn = &mut data.conn_pool.get().unwrap();
        mon_ids = filter.get_matching_mons(conn);
    }

    let (stats, lvl_avg) = {
        use schema::battles;
        use schema::trainer_mon_stats;
        use schema::trainer_mons;

        let conn = &mut data.conn_pool.get().unwrap();

        // Get all trainers with those mons
        let trainers = trainer_mons::table
            .select(trainer_mons::trainer_id)
            .filter(trainer_mons::mon_id.eq_any(&mon_ids));

        let stats: Vec<(
            i32,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
        )> = trainer_mon_stats::table
            .group_by(trainer_mon_stats::mon_id)
            .select((
                trainer_mon_stats::mon_id,
                sum(trainer_mon_stats::murders),
                sum(trainer_mon_stats::deaths),
                sum(trainer_mon_stats::times_released),
                sum(trainer_mon_stats::damage_dealt),
                sum(trainer_mon_stats::damage_taken),
            ))
            .inner_join(battles::table.on(battles::id.eq(trainer_mon_stats::battle_id)))
            .filter(
                battles::player_perspective
                    .eq_any(trainers.clone())
                    .and(battles::opponent_perspective.eq_any(trainers.clone())),
            )
            .load(conn)
            .expect("Error loading series");

        let lvl_avg: Vec<(i32, Option<i64>, i64)> = trainer_mons::table
            .filter(trainer_mons::mon_id.eq_any(&mon_ids))
            .group_by(trainer_mons::mon_id)
            .select((
                trainer_mons::mon_id,
                sum(trainer_mons::lvl),
                count(trainer_mons::mon_id),
            ))
            .load(conn)
            .expect("Error loading series");

        (stats, lvl_avg)
    };

    let lvl_avg: HashMap<_, _> = lvl_avg
        .into_iter()
        .map(|(id, lvl_sum, mon_count)| (id, (lvl_sum, mon_count)))
        .collect();
    let stats: HashMap<_, _> = stats
        .into_iter()
        .map(|(id, m, d, tr, dd, dt)| (id, (m, d, tr, dd, dt)))
        .collect();

    let mut result: Vec<MonRanking> = vec![];
    for mon_id in mon_ids {
        let mut rank = MonRanking {
            mon_id,
            murders: 0,
            deaths: 0,
            times_released: 0,
            damage_dealt: 0,
            damage_taken: 0,
            average_level: 0.0,
            number_exist: 0,
        };

        if let Some((murders, deaths, times_released, damage_dealt, damage_taken)) =
            stats.get(&mon_id)
        {
            rank.murders = murders.unwrap_or_default();
            rank.deaths = deaths.unwrap_or_default();
            rank.times_released = times_released.unwrap_or_default();
            rank.damage_dealt = damage_dealt.unwrap_or_default();
            rank.damage_taken = damage_taken.unwrap_or_default();
        }

        if let Some((lvl_sum, mon_count)) = lvl_avg.get(&mon_id) {
            rank.average_level = lvl_sum.unwrap_or_default() as f64 / *mon_count as f64;
            rank.number_exist = *mon_count;
        }

        result.push(rank);
    }

    // Sort by kd ratio
    result.sort_by(|a, b| {
        let a_kd = if a.deaths == 0 {
            a.murders as f64
        } else {
            a.murders as f64 / a.deaths as f64
        };

        let b_kd = if b.deaths == 0 {
            b.murders as f64
        } else {
            b.murders as f64 / b.deaths as f64
        };

        a_kd.partial_cmp(&b_kd)
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });

    return web::Json(MonRankResponse { mons: result });
}

pub(crate) fn scope() -> Scope {
    web::scope("/mon")
        .service(get_series)
        .service(get_stats)
        .service(get_rank)
}

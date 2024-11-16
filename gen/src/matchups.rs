use log::{debug, info};
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use actix_web::cookie::time::Duration;
use all_battle_core::{
    gen3::GEN3,
    models::{self, Battle},
};
use diesel::prelude::*;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

use crate::runner::Game;

#[derive(Debug)]
pub struct Error {}

#[derive(Clone, Deserialize, Serialize)]
pub struct PendingBattle {
    pub series_id: i32,
    pub round: i32,
    pub player: u16,
    pub battler: u16,
    pub seed: u16,
}

fn trim_logging_prefix(line: &str) -> &str {
    if let Some(prefix) = line.find("[mGBA mLOG]: ") {
        &line[prefix + "[mGBA mLOG]: ".len()..]
    } else {
        line
    }
}

const PLAYER_CHOOSE_MON: &str = "PP";
const OPPONENT_CHOOSE_MON: &str = "OP";
const PLAYER_FAINT: &str = "PF";
const OPPONENT_FAINT: &str = "OF";
const PLAYER_DAMAGED: &str = "PD";
const OPPONENT_DAMAGED: &str = "OD";
const PLAYER_CHOOSE_MOVE: &str = "PM";
const OPPONENT_CHOOSE_MOVE: &str = "OM";

impl PendingBattle {
    pub fn run(&self, game: &Game) -> Result<BattleResult, Error> {
        info!(
            "{} Running battle {} vs {}",
            game.id, self.player, self.battler
        );

        game.send_write_u16(game.player_address, self.player);
        game.send_write_u16(game.battler_address, self.battler);

        let mut player_won: Option<bool> = None;
        let mut key_up_time = None;

        let mut last_time = std::time::Instant::now();
        let mut elapsed = Duration::ZERO;

        let mut events = vec![
            format!("0.0,{PLAYER_CHOOSE_MON},0"),
            format!("0.0,{OPPONENT_CHOOSE_MON},0"),
        ];

        loop {
            let line = match game.out.recv() {
                Ok(line) => line,
                Err(_) => {
                    panic!("Game process died {} vs {}", self.player, self.battler);
                }
            };
            let line = trim_logging_prefix(&line).to_string();
            elapsed += std::time::Instant::now() - last_time;
            last_time = std::time::Instant::now();
            debug!(
                "{} {} vs {} \"{}\"",
                game.id, self.player, self.battler, line
            );
            if let Some(time) = key_up_time {
                if time < std::time::Instant::now() {
                    game.send_key_up("A");
                    key_up_time = None;
                }
            }

            if line.contains("CheckTrainer Ready") {
                if key_up_time.is_none() {
                    game.send_key_down("A");
                    key_up_time =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(250));
                } else if player_won.is_some() {
                    break;
                }
            } else if line.contains("ScrCmd_dotrainerbattle") {
                game.send_write_u16(game.rng_address, self.seed);
            } else if line.contains("CB2_InitBattle") {
                game.send_start_recording();
                elapsed = Duration::ZERO;
            } else if line.contains("Task_HandleMainMenuInput") {
                game.send_key_down("A");
            } else if line.contains("HandleEndTurn_BattleLost") {
                player_won = Some(false);
            } else if line.contains("HandleEndTurn_BattleWon") {
                player_won = Some(true);
            } else if line.contains("ReturnFromBattleToOverworld") {
                break;
            } else if line.contains("!PlayerChooseMon") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{PLAYER_CHOOSE_MON},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!OpponentChooseMon") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{OPPONENT_CHOOSE_MON},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!PlayerMonFainted") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!("{},{PLAYER_FAINT},{}", elapsed.as_seconds_f32(), splits[0])
                        .to_string(),
                );
            } else if line.contains("!OpponentMonFainted") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{OPPONENT_FAINT},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!PlayerDamagedValue") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{PLAYER_DAMAGED},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!OpponentDamagedValue") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{OPPONENT_DAMAGED},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!PlayerChooseMove") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{PLAYER_CHOOSE_MOVE},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            } else if line.contains("!OpponentChooseMove") {
                let splits: Vec<_> = line.split(" ").collect();
                events.push(
                    format!(
                        "{},{OPPONENT_CHOOSE_MOVE},{}",
                        elapsed.as_seconds_f32(),
                        splits[0]
                    )
                    .to_string(),
                );
            }
        }

        debug!(
            "{} Battle completed {} vs {} saving video",
            game.id, self.player, self.battler
        );
        let video_file = self.video_filename(self.round);

        game.send_stop_recording(
            &game
                .video_path
                .join(&video_file)
                .to_str()
                .unwrap()
                .to_string(),
        );

        debug!(
            "{} Battle completed {} vs {} video saved",
            game.id, self.player, self.battler
        );

        let player_won = player_won.unwrap();

        Ok(BattleResult {
            player_won,
            video_file,
            events,
            duration: elapsed.as_seconds_f32() as i32,
        })
    }

    fn video_filename(&self, round: i32) -> String {
        let (a, b) = if self.player < self.battler {
            (self.player, self.battler)
        } else {
            (self.battler, self.player)
        };

        format!("{}-{}_{}.mp4", a, b, round)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BattleResult {
    pub player_won: bool,
    pub video_file: String,
    pub duration: i32,
    pub events: Vec<String>,
}

impl BattleResult {
    pub fn save(&self, conn: &mut SqliteConnection, pending_battle: &PendingBattle) -> Battle {
        use all_battle_core::schema::battles;

        diesel::insert_into(battles::table)
            .values(&models::NewBattle {
                series_id: pending_battle.series_id,
                player_perspective: pending_battle.player as i32,
                opponent_perspective: pending_battle.battler as i32,
                player_perspective_won: self.player_won,
                seed: pending_battle.seed as i32,
                created_at: chrono::offset::Utc::now().timestamp_millis(),
                video_path: self.video_file.clone(),
                duration: self.duration,
                events: self.events.join(":"),
            })
            .get_result(conn)
            .expect("Error saving new battle")
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct TrainerBattleMonMoveStats {
    move_id: i32,
    target_index: i32,
    times_used: i32,
    murders: i32,
    damage_dealt: i32,
}

#[derive(Default, Clone)]
struct TrainerBattleMonStats {
    mon_id: i32,
    damage_dealt: i32,
    damage_taken: i32,
    times_released: i32,
    murders: i32,
    deaths: i32,
    moves: Vec<TrainerBattleMonMoveStats>,
}

impl TrainerBattleMonStats {
    pub fn get_move_mut(
        &mut self,
        move_id: i32,
        target_index: i32,
    ) -> &mut TrainerBattleMonMoveStats {
        for i in 0..self.moves.len() {
            if self.moves[i].move_id == move_id {
                return self.moves.get_mut(i).unwrap();
            }
        }

        // Sometimes shits just fucked and we have copy moves struggle
        // god knows what else just add a new entry if it doesn't exist
        self.moves.push(TrainerBattleMonMoveStats {
            move_id,
            target_index,
            ..Default::default()
        });
        self.moves.last_mut().unwrap()
    }
}

fn insert_mon_stats(
    conn: Arc<Mutex<SqliteConnection>>,
    battle_id: i32,
    trainer_id: i32,
    party: &[TrainerBattleMonStats],
) {
    use all_battle_core::schema::trainer_mon_stats;
    use all_battle_core::schema::trainer_mons;

    let mut inserts = vec![];

    for (i, mon) in party.iter().enumerate() {
        if mon.mon_id == 0 {
            continue;
        }

        let trainer_mon_id: i32 = {
            let mut lock = conn.lock().unwrap();
            let conn = lock.deref_mut();

            let trainer_mon_id = trainer_mons::table
                .select(trainer_mons::id)
                .filter(trainer_mons::trainer_id.eq(trainer_id))
                .filter(trainer_mons::party_index.eq(i as i32))
                .first(conn)
                .expect("Error saving new mon stat");

            trainer_mon_id
        };

        inserts.push(models::NewTrainerMonStat {
            trainer_id,
            trainer_mon_id,
            battle_id,
            mon_id: mon.mon_id,
            damage_dealt: mon.damage_dealt,
            damage_taken: mon.damage_taken,
            times_released: mon.times_released,
            murders: mon.murders,
            deaths: mon.deaths,
        });
    }

    {
        let mut lock = conn.lock().unwrap();
        let conn = lock.deref_mut();

        diesel::insert_into(trainer_mon_stats::table)
            .values(inserts)
            .execute(conn)
            .expect("Error saving new mon stat");
    };
}

fn insert_mon_move_stats(
    conn: Arc<Mutex<SqliteConnection>>,
    battle_id: i32,
    trainer_id: i32,
    other_trainer_id: i32,
    party: &[TrainerBattleMonStats],
    other_party: &[TrainerBattleMonStats],
) {
    use all_battle_core::schema::trainer_mon_stats;
    use all_battle_core::schema::trainer_mon_stats_moves;
    use all_battle_core::schema::trainer_mons;

    let mut inserts = vec![];

    let mut lock = conn.lock().unwrap();
    let conn = lock.deref_mut();

    for (i, mon) in party.iter().enumerate() {
        for mon_move in &mon.moves {
            if mon_move.move_id == 0 {
                continue;
            }

            let mon_stat_id = trainer_mon_stats::table
                .select(trainer_mon_stats::id)
                .inner_join(
                    trainer_mons::table.on(trainer_mons::id.eq(trainer_mon_stats::trainer_mon_id)),
                )
                .filter(trainer_mon_stats::trainer_id.eq(trainer_id))
                .filter(trainer_mons::party_index.eq(i as i32))
                .filter(trainer_mon_stats::battle_id.eq(battle_id))
                .filter(trainer_mon_stats::mon_id.eq(mon.mon_id))
                .first::<i32>(conn)
                .expect("Error loading mon stat");

            let target_mon_stat_id = trainer_mon_stats::table
                .select(trainer_mon_stats::id)
                .inner_join(
                    trainer_mons::table.on(trainer_mons::id.eq(trainer_mon_stats::trainer_mon_id)),
                )
                .filter(trainer_mon_stats::trainer_id.eq(other_trainer_id))
                .filter(trainer_mons::party_index.eq(mon_move.target_index))
                .filter(trainer_mon_stats::battle_id.eq(battle_id))
                .filter(
                    trainer_mon_stats::mon_id
                        .eq(other_party[mon_move.target_index as usize].mon_id),
                )
                .first::<i32>(conn)
                .expect("Error loading mon stat");

            inserts.push(models::NewTrainerMonStatMove {
                trainer_mon_stats_id: mon_stat_id,
                trainer_mon_target_id: target_mon_stat_id,
                move_id: mon_move.move_id,
                times_used: mon_move.times_used,
                damage_dealt: mon_move.damage_dealt,
                murders: mon_move.murders,
            });
        }
    }

    diesel::insert_into(trainer_mon_stats_moves::table)
        .values(&inserts)
        .execute(conn)
        .expect("Error saving new mon stat move");
}

fn init_trainer_stats(trainer_id: i32) -> Vec<TrainerBattleMonStats> {
    let player = GEN3.get_trainer_by_id(trainer_id);
    let mut result = vec![TrainerBattleMonStats::default(); player.party_size as usize];
    for (i, trainer_mon) in player.party.iter().enumerate() {
        let mon = GEN3.get_mon_by_species(&trainer_mon.species);
        result[i].mon_id = mon.id;
    }
    result
}

fn update_post_battle_stats(
    conn: Arc<Mutex<SqliteConnection>>,
    battle: &Battle,
    events: &[String],
) {
    debug!(
        "Updating battle {} vs {} stats",
        battle.player_perspective, battle.opponent_perspective
    );

    let mut player_party = init_trainer_stats(battle.player_perspective);
    let mut opponent_party = init_trainer_stats(battle.opponent_perspective);

    let mut player_mon_index: Option<usize> = Some(0);
    let mut player_move_id: i32 = 0;
    let mut opponent_mon_index: Option<usize> = Some(0);
    let mut opponent_move_id: i32 = 0;

    for event in events {
        let splits = event.split(",").collect::<Vec<_>>();
        let _time = splits[0].parse::<f32>().unwrap();
        let event = splits[1];

        debug!(
            "{} vs {} Processing Event {} {:?}",
            battle.player_perspective, battle.opponent_perspective, event, splits
        );
        match event {
            PLAYER_CHOOSE_MON => {
                let mon_index = splits[2].parse::<usize>().unwrap();
                if mon_index >= player_party.len() {
                    player_mon_index = None;
                    continue;
                }
                player_mon_index = Some(mon_index);
                player_party[player_mon_index.unwrap()].times_released = 1;
            }
            OPPONENT_CHOOSE_MON => {
                let mon_index = splits[2].parse::<usize>().unwrap();
                if mon_index >= opponent_party.len() {
                    opponent_mon_index = None;
                    continue;
                }
                opponent_mon_index = Some(mon_index);
                opponent_party[opponent_mon_index.unwrap()].times_released = 1;
            }
            PLAYER_FAINT => {
                if let Some(player_mon_index) = player_mon_index {
                    player_party[player_mon_index].deaths += 1;
                }

                if let Some(opponent_mon_index) = opponent_mon_index {
                    opponent_party[opponent_mon_index].murders += 1;
                    if let Some(player_mon_index) = player_mon_index {
                        opponent_party[opponent_mon_index]
                            .get_move_mut(opponent_move_id, player_mon_index as i32)
                            .murders += 1;
                    }
                }
            }
            OPPONENT_FAINT => {
                if let Some(opponent_mon_index) = opponent_mon_index {
                    opponent_party[opponent_mon_index].deaths += 1;
                }

                if let Some(player_mon_index) = player_mon_index {
                    player_party[player_mon_index].murders += 1;
                    if let Some(opponent_mon_index) = opponent_mon_index {
                        player_party[player_mon_index]
                            .get_move_mut(player_move_id, opponent_mon_index as i32)
                            .murders += 1;
                    }
                }
            }
            PLAYER_DAMAGED => {
                let damage = splits[2].parse::<i32>().unwrap();
                if damage > 0 {
                    if let Some(player_mon_index) = player_mon_index {
                        player_party[player_mon_index].damage_taken += damage;
                    }

                    if let Some(opponent_mon_index) = opponent_mon_index {
                        opponent_party[opponent_mon_index].damage_dealt += damage;
                        if let Some(player_mon_index) = player_mon_index {
                            opponent_party[opponent_mon_index]
                                .get_move_mut(opponent_move_id, player_mon_index as i32)
                                .damage_dealt += damage;
                        }
                    }
                }
            }
            OPPONENT_DAMAGED => {
                let damage = splits[2].parse::<i32>().unwrap();
                if damage > 0 {
                    if let Some(opponent_mon_index) = opponent_mon_index {
                        opponent_party[opponent_mon_index].damage_taken += damage;
                    }

                    if let Some(player_mon_index) = player_mon_index {
                        player_party[player_mon_index].damage_dealt += damage;
                        if let Some(opponent_mon_index) = opponent_mon_index {
                            player_party[player_mon_index]
                                .get_move_mut(player_move_id, opponent_mon_index as i32)
                                .damage_dealt += damage;
                        }
                    }
                }
            }
            PLAYER_CHOOSE_MOVE => {
                let move_id = splits[2].parse::<i32>().unwrap();
                player_move_id = move_id;
                if let Some(player_mon_index) = player_mon_index {
                    if let Some(opponent_mon_index) = opponent_mon_index {
                        player_party[player_mon_index]
                            .get_move_mut(player_move_id, opponent_mon_index as i32)
                            .times_used += 1;
                    }
                }
            }
            OPPONENT_CHOOSE_MOVE => {
                let move_id = splits[2].parse::<i32>().unwrap();
                opponent_move_id = move_id;
                if let Some(opponent_mon_index) = opponent_mon_index {
                    if let Some(player_mon_index) = player_mon_index {
                        opponent_party[opponent_mon_index]
                            .get_move_mut(opponent_move_id, player_mon_index as i32)
                            .times_used += 1;
                    }
                }
            }
            _ => {}
        }
    }

    debug!(
        "Updating battle {} vs {} stats mon stats player",
        battle.player_perspective, battle.opponent_perspective
    );

    insert_mon_stats(
        conn.clone(),
        battle.id,
        battle.player_perspective,
        &player_party,
    );

    debug!(
        "Updating battle {} vs {} stats mon stats opponent",
        battle.player_perspective, battle.opponent_perspective
    );

    insert_mon_stats(
        conn.clone(),
        battle.id,
        battle.opponent_perspective,
        &opponent_party,
    );

    debug!(
        "Updating battle {} vs {} stats mon move opponent",
        battle.player_perspective, battle.opponent_perspective
    );

    insert_mon_move_stats(
        conn.clone(),
        battle.id,
        battle.player_perspective,
        battle.opponent_perspective,
        &player_party,
        &opponent_party,
    );

    info!(
        "Updating battle {} vs {} stats mon move player",
        battle.player_perspective, battle.opponent_perspective
    );

    insert_mon_move_stats(
        conn.clone(),
        battle.id,
        battle.opponent_perspective,
        battle.player_perspective,
        &opponent_party,
        &player_party,
    );

    info!(
        "Battle {} vs {} stats updated",
        battle.player_perspective, battle.opponent_perspective
    );
}

pub fn complete_series(conn: Arc<Mutex<SqliteConnection>>, game: &Game, series_id: i32) {
    use all_battle_core::schema::battles;
    use all_battle_core::schema::series;
    use all_battle_core::schema::series_trainers;

    debug!("{} Querying series and trainers {}", game.id, series_id);

    let (series, trainers, mut completed_battles) = {
        let mut lock = conn.lock().unwrap();

        let conn = lock.deref_mut();

        let series: models::Series = series::table
            .filter(series::id.eq(series_id))
            .first::<models::Series>(conn)
            .expect("Error loading series");

        debug!("{} Got series {}", game.id, series_id);

        let series_trainers: Vec<i32> = series_trainers::table
            .filter(series_trainers::series_id.eq(series_id))
            .select(series_trainers::trainer_id)
            .load::<i32>(conn)
            .expect("Error loading series trainers");

        debug!("{} Got series trainers", game.id);

        let completed_battles: Vec<models::Battle> = battles::table
            .filter(battles::series_id.eq(series_id))
            .order_by(battles::created_at.asc())
            .load::<models::Battle>(conn)
            .expect("Error loading battles");

        debug!("{} Got series battles", game.id);

        (series, series_trainers, completed_battles)
    };

    debug!("{} Got completed battles for series {}", game.id, series_id);

    let mut rng = rand::thread_rng();

    loop {
        // Tally existing wins
        let most_wins = {
            if completed_battles.is_empty() {
                0
            } else {
                let mut wins: HashMap<i32, i32> = HashMap::new();

                for battle in &completed_battles {
                    let winner = if battle.player_perspective_won {
                        battle.player_perspective
                    } else {
                        battle.opponent_perspective
                    };

                    let count = wins.entry(winner).or_insert(0);
                    *count += 1;
                }

                *wins.values().max().unwrap()
            }
        };

        debug!("{} Series {} most wins {}", game.id, series_id, most_wins);

        if most_wins >= series.first_to {
            {
                let mut lock = conn.lock().unwrap();
                let conn = lock.deref_mut();

                diesel::update(series::table)
                    .filter(series::id.eq(series_id))
                    .set(series::completed_at.eq(chrono::offset::Utc::now().timestamp_millis()))
                    .execute(conn)
                    .expect("Error updating series");
            }
            break;
        }

        let (player, opponent) = if let Some(last_battle) = completed_battles.last() {
            (
                last_battle.opponent_perspective,
                last_battle.player_perspective,
            )
        } else {
            // Just pick it randomly
            if rng.gen_bool(0.5) {
                (trainers[0], trainers[1])
            } else {
                (trainers[1], trainers[0])
            }
        };

        let pending_battle = PendingBattle {
            series_id,
            round: completed_battles.len() as i32 + 1,
            player: player as u16,
            battler: opponent as u16,
            seed: rng.gen(),
        };

        match pending_battle.run(game) {
            Ok(result) => {
                info!(
                    "{} Battle completed {} vs {} player won {}",
                    game.id, pending_battle.player, pending_battle.battler, result.player_won
                );
                let battle = result.save(&mut conn.lock().unwrap(), &pending_battle);
                update_post_battle_stats(conn.clone(), &battle, &result.events);
                completed_battles.push(battle);
            }
            Err(err) => {
                info!("{} Error running battle: {:?}", game.id, err);
                continue;
            }
        }
    }
}

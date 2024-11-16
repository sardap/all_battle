use diesel::prelude::*;
use std::collections::HashSet;

use lazy_static::lazy_static;
use log::{debug, info};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainerMon {
    pub iv: i32,
    pub lvl: i32,
    pub species: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moves: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainerParty {
    pub name: String,
    pub mons: Vec<TrainerMon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trainer {
    pub id: i32,
    #[serde(rename = "idName")]
    pub id_name: String,
    #[serde(rename = "partyFlags")]
    pub party_flags: u8,
    #[serde(rename = "trainerClass")]
    pub trainer_class: String,
    #[serde(rename = "encounterMusic_gender")]
    pub encounter_music_gender: String,
    #[serde(rename = "trainerPic")]
    pub trainer_pic: String,
    #[serde(rename = "trainerName")]
    pub trainer_name: String,
    pub items: Vec<String>,
    #[serde(rename = "doubleBattle")]
    pub double_battle: bool,
    #[serde(rename = "aiFlags")]
    pub ai_flags: Vec<String>,
    #[serde(rename = "partySize")]
    pub party_size: u8,
    pub party: Vec<TrainerMon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    pub id: i32,
    #[serde(rename = "idName")]
    pub id_name: String,
    pub name: String,
    pub effect: String,
    pub power: u16,
    #[serde(rename = "type")]
    pub move_type: String,
    pub accuracy: u8,
    pub pp: u8,
    #[serde(rename = "secondaryEffectChance")]
    pub secondary_effect_chance: u8,
    pub target: String,
    pub priority: i8,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelUpMove {
    pub level: i32,
    #[serde(rename = "move")]
    pub move_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mon {
    pub id: i32,
    #[serde(rename = "idName")]
    pub id_name: String,
    pub name: String,
    #[serde(rename = "levelUpMoves")]
    pub level_up_moves: Vec<LevelUpMove>,
    #[serde(rename = "monTypes")]
    pub mon_types: Vec<String>,
    pub abilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gen3Output {
    pub trainers: Vec<Trainer>,
    pub moves: Vec<Move>,
    pub mons: Vec<Mon>,
}

impl Gen3Output {
    pub fn get_trainer_by_id(&self, id: i32) -> &Trainer {
        self.trainers
            .iter()
            .find(|t| t.id == id)
            .unwrap_or(&self.trainers[0])
    }

    pub fn get_move_by_id(&self, id: i32) -> &Move {
        self.moves
            .iter()
            .find(|m: &&Move| m.id == id)
            .unwrap_or(&self.moves[0])
    }

    pub fn get_move_by_name(&self, name: &str) -> &Move {
        match self.moves.iter().find(|m| m.id_name == name) {
            Some(m) => m,
            None => &self.moves[0],
        }
    }

    pub fn get_mon_by_id(&self, id: i32) -> &Mon {
        self.mons
            .iter()
            .find(|m| m.id == id)
            .unwrap_or(&self.mons[0])
    }

    pub fn get_mon_by_species(&self, species: &str) -> &Mon {
        self.mons
            .iter()
            .find(|m| m.id_name == species)
            .unwrap_or(&self.mons[0])
    }

    pub fn get_moves_for_trainer_mon(&self, mon: &TrainerMon) -> Vec<&Move> {
        if let Some(moves) = &mon.moves {
            return moves.iter().map(|m| self.get_move_by_name(m)).collect();
        }

        let lvl = mon.lvl;

        let mon = self.get_mon_by_species(&mon.species);
        let mut moves = vec![];
        for level_up_move in mon.level_up_moves.iter().rev() {
            if level_up_move.level <= lvl {
                moves.push(self.get_move_by_name(&level_up_move.move_name));
                if moves.len() == 4 {
                    break;
                }
            }
        }

        moves
    }
}

lazy_static! {
    pub static ref GEN3: Gen3Output = {
        let raw = include_bytes!("gen3.json");
        let result: Gen3Output = serde_json::from_slice(raw).unwrap();
        result
    };
    pub static ref GEN3_TRAINER_MONS_IDS: Vec<i32> = {
        let mut mons = HashSet::new();
        for trainer in GEN3.trainers.iter() {
            for mon in trainer.party.iter() {
                mons.insert(GEN3.get_mon_by_species(&mon.species).id);
            }
        }
        let mut result: Vec<_> = mons.into_iter().collect();
        result.sort();
        result
    };
}

pub fn populate_gen3_tables(conn: &mut SqliteConnection) {
    use crate::schema::mon_abilities;
    use crate::schema::mon_level_up_moves;
    use crate::schema::mon_type_mapping;
    use crate::schema::mon_types;
    use crate::schema::mons;
    use crate::schema::move_flags;
    use crate::schema::moves;
    use crate::schema::trainer_mon_moves;
    use crate::schema::trainer_mons;

    if mon_types::table.count().get_result::<i64>(conn).unwrap() > 0 {
        info!("Gen3 tables already populated");
        return;
    }

    info!("Populating gen3 tables");

    info!("Populating gen3 types");
    {
        let mut mon_types = HashSet::new();
        for mon_move in GEN3.moves.iter() {
            mon_types.insert(&mon_move.move_type);
        }
        for mon_type in mon_types {
            diesel::insert_into(mon_types::table)
                .values(mon_types::name.eq(mon_type))
                .execute(conn)
                .unwrap();
        }
    }

    info!("Populating gen3 moves");
    {
        let mut moves_to_insert = vec![];
        let mut move_flags_to_insert = vec![];

        for mon_move in &GEN3.moves {
            moves_to_insert.push((
                moves::id.eq(mon_move.id),
                moves::id_name.eq(&mon_move.id_name),
                moves::name.eq(&mon_move.name),
                moves::effect.eq(&mon_move.effect),
                moves::power.eq(mon_move.power as i32),
                moves::move_type.eq(&mon_move.move_type),
                moves::accuracy.eq(mon_move.accuracy as i32),
                moves::pp.eq(mon_move.pp as i32),
                moves::secondary_effect_chance.eq(mon_move.secondary_effect_chance as i32),
                moves::target.eq(&mon_move.target),
                moves::priority.eq(mon_move.priority as i32),
            ));

            for flag in &mon_move.flags {
                move_flags_to_insert.push((
                    move_flags::move_id.eq(mon_move.id),
                    move_flags::flag.eq(flag),
                ));
            }
        }

        diesel::insert_into(moves::table)
            .values(moves_to_insert)
            .execute(conn)
            .expect("Error inserting moves");

        diesel::insert_into(move_flags::table)
            .values(move_flags_to_insert)
            .execute(conn)
            .expect("Error inserting move flags");
    }

    info!("Populating gen3 mons");
    {
        let mut mons_to_insert = vec![];
        let mut mon_type_mappings_to_insert = vec![];
        let mut mon_level_up_moves_to_insert = vec![];
        let mut mon_abilities_to_insert = vec![];

        for mon in &GEN3.mons {
            mons_to_insert.push((
                mons::id.eq(mon.id),
                mons::id_name.eq(&mon.id_name),
                mons::name.eq(&mon.name),
            ));

            for mon_type in &mon.mon_types {
                mon_type_mappings_to_insert.push((
                    mon_type_mapping::mon_id.eq(mon.id),
                    mon_type_mapping::mon_type.eq(mon_type),
                ));
            }

            for level_up_move in &mon.level_up_moves {
                mon_level_up_moves_to_insert.push((
                    mon_level_up_moves::mon_id.eq(mon.id),
                    mon_level_up_moves::move_id
                        .eq(GEN3.get_move_by_name(&level_up_move.move_name).id),
                    mon_level_up_moves::lvl.eq(level_up_move.level),
                ));
            }

            for ability in &mon.abilities {
                mon_abilities_to_insert.push((
                    mon_abilities::mon_id.eq(mon.id),
                    mon_abilities::ability.eq(ability),
                ));
            }
        }

        diesel::insert_into(mons::table)
            .values(mons_to_insert)
            .execute(conn)
            .expect("Error inserting mons");

        diesel::insert_into(mon_type_mapping::table)
            .values(mon_type_mappings_to_insert)
            .execute(conn)
            .expect("Error inserting mon type mappings");

        diesel::insert_into(mon_level_up_moves::table)
            .values(mon_level_up_moves_to_insert)
            .execute(conn)
            .unwrap();

        diesel::insert_or_ignore_into(mon_abilities::table)
            .values(mon_abilities_to_insert)
            .execute(conn)
            .unwrap();
    }

    info!("Populating gen3 trainers");

    info!("Populating gen3 trainer mons");
    {
        let mut trainer_mons_to_insert = vec![];

        for trainer in &GEN3.trainers {
            debug!("Populating mons for trainer {}", trainer.id);
            for (i, mon) in trainer.party.iter().enumerate() {
                debug!("Populating mon {} for trainer {}", i, trainer.id);
                let mon_id = GEN3.get_mon_by_species(&mon.species).id;
                trainer_mons_to_insert.push((
                    trainer_mons::trainer_id.eq(trainer.id),
                    trainer_mons::party_index.eq(i as i32),
                    trainer_mons::iv.eq(mon.iv),
                    trainer_mons::lvl.eq(mon.lvl),
                    trainer_mons::mon_id.eq(mon_id),
                    trainer_mons::held_item.eq(mon.held_item.as_ref()),
                ));
            }
        }
        diesel::insert_into(trainer_mons::table)
            .values(&trainer_mons_to_insert)
            .execute(conn)
            .unwrap();

        let mut trainer_mon_moves = vec![];

        for trainer in &GEN3.trainers {
            for (i, mon) in trainer.party.iter().enumerate() {
                let trainer_mon_id = trainer_mons::table
                    .select(trainer_mons::id)
                    .filter(trainer_mons::trainer_id.eq(trainer.id))
                    .filter(trainer_mons::party_index.eq(i as i32))
                    .first::<i32>(conn)
                    .unwrap();

                if let Some(moves) = &mon.moves {
                    for (j, mon_move) in moves.iter().enumerate() {
                        let mon_move = GEN3.get_move_by_name(mon_move);
                        debug!(
                            "Populating mon {} move {} move_id {} for trainer {}",
                            i, j, mon_move.id, trainer.id
                        );
                        trainer_mon_moves.push((
                            trainer_mon_moves::trainer_mon_id.eq(trainer_mon_id),
                            trainer_mon_moves::move_id.eq(mon_move.id),
                        ));
                    }
                }
            }
        }

        diesel::insert_into(trainer_mon_moves::table)
            .values(trainer_mon_moves)
            .execute(conn)
            .unwrap();
    }

    info!("Done Populating gen3 tables");
}

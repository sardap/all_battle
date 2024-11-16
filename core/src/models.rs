use diesel::prelude::*;
use serde_derive::Serialize;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::trainers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Trainer {
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTournament {
    pub name: String,
    pub tournament_type: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub tournament_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::series)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSeries {
    pub tournament_id: i32,
    pub priority: i32,
    pub first_to: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::series)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Series {
    pub id: i32,
    pub tournament_id: i32,
    pub first_to: i32,
    pub priority: i32,
    pub completed_at: Option<i64>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::series_trainers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SeriesTrainer {
    pub series_id: i32,
    pub trainer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::battles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewBattle {
    pub series_id: i32,
    pub created_at: i64,
    pub player_perspective: i32,
    pub opponent_perspective: i32,
    pub player_perspective_won: bool,
    pub seed: i32,
    pub video_path: String,
    pub duration: i32,
    pub events: String,
}

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = crate::schema::battles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Battle {
    pub id: i32,
    pub series_id: i32,
    pub created_at: i64,
    pub player_perspective: i32,
    pub opponent_perspective: i32,
    pub player_perspective_won: bool,
    pub seed: i32,
    pub video_path: String,
    pub duration: i32,
    pub events: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::trainer_mon_stats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTrainerMonStat {
    pub trainer_id: i32,
    pub trainer_mon_id: i32,
    pub battle_id: i32,
    pub mon_id: i32,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub times_released: i32,
    pub murders: i32,
    pub deaths: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::trainer_mon_stats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerMonStat {
    pub id: i32,
    pub trainer_id: i32,
    pub trainer_mon_id: i32,
    pub battle_id: i32,
    pub mon_id: i32,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub times_released: i32,
    pub murders: i32,
    pub deaths: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::trainer_mon_stats_moves)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTrainerMonStatMove {
    pub trainer_mon_stats_id: i32,
    pub trainer_mon_target_id: i32,
    pub move_id: i32,
    pub times_used: i32,
    pub damage_dealt: i32,
    pub murders: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::trainer_mon_stats_moves)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TrainerMonStatMove {
    pub id: i32,
    pub trainer_mon_stats_id: i32,
    pub trainer_mon_target_id: i32,
    pub move_id: i32,
    pub times_used: i32,
    pub damage_dealt: i32,
    pub murders: i32,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::mon_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MonType {
    pub name: String,
}

// @generated automatically by Diesel CLI.

diesel::table! {
    battles (id) {
        id -> Integer,
        series_id -> Integer,
        created_at -> BigInt,
        player_perspective -> Integer,
        opponent_perspective -> Integer,
        player_perspective_won -> Bool,
        seed -> Integer,
        video_path -> Text,
        duration -> Integer,
        events -> Text,
    }
}

diesel::table! {
    mon_abilities (mon_id, ability) {
        mon_id -> Integer,
        ability -> Text,
    }
}

diesel::table! {
    mon_level_up_moves (mon_id, move_id, lvl) {
        mon_id -> Integer,
        move_id -> Integer,
        lvl -> Integer,
    }
}

diesel::table! {
    mon_type_mapping (rowid) {
        rowid -> Integer,
        mon_id -> Integer,
        mon_type -> Text,
    }
}

diesel::table! {
    mon_types (name) {
        name -> Text,
    }
}

diesel::table! {
    mons (id) {
        id -> Integer,
        id_name -> Text,
        name -> Text,
    }
}

diesel::table! {
    move_flags (move_id, flag) {
        move_id -> Integer,
        flag -> Text,
    }
}

diesel::table! {
    moves (id) {
        id -> Integer,
        id_name -> Text,
        name -> Text,
        effect -> Text,
        power -> Integer,
        move_type -> Text,
        accuracy -> Integer,
        pp -> Integer,
        secondary_effect_chance -> Integer,
        target -> Text,
        priority -> Integer,
    }
}

diesel::table! {
    series (id) {
        id -> Integer,
        tournament_id -> Integer,
        first_to -> Integer,
        priority -> Integer,
        completed_at -> Nullable<BigInt>,
    }
}

diesel::table! {
    series_trainers (series_id, trainer_id) {
        series_id -> Integer,
        trainer_id -> Integer,
    }
}

diesel::table! {
    tournaments (id) {
        id -> Integer,
        name -> Text,
        tournament_type -> Text,
    }
}

diesel::table! {
    trainer_mon_moves (trainer_mon_id, move_id) {
        trainer_mon_id -> Integer,
        move_id -> Integer,
    }
}

diesel::table! {
    trainer_mon_stats (id) {
        id -> Integer,
        trainer_id -> Integer,
        trainer_mon_id -> Integer,
        battle_id -> Integer,
        mon_id -> Integer,
        damage_dealt -> Integer,
        damage_taken -> Integer,
        times_released -> Integer,
        murders -> Integer,
        deaths -> Integer,
    }
}

diesel::table! {
    trainer_mon_stats_moves (id) {
        id -> Integer,
        trainer_mon_stats_id -> Integer,
        trainer_mon_target_id -> Integer,
        move_id -> Integer,
        times_used -> Integer,
        damage_dealt -> Integer,
        murders -> Integer,
    }
}

diesel::table! {
    trainer_mons (id) {
        id -> Integer,
        trainer_id -> Integer,
        party_index -> Integer,
        iv -> Integer,
        lvl -> Integer,
        mon_id -> Integer,
        held_item -> Nullable<Text>,
    }
}

diesel::table! {
    trainers (id) {
        id -> Integer,
    }
}

diesel::joinable!(battles -> series (series_id));
diesel::joinable!(mon_abilities -> mons (mon_id));
diesel::joinable!(mon_level_up_moves -> mons (mon_id));
diesel::joinable!(mon_level_up_moves -> moves (move_id));
diesel::joinable!(mon_type_mapping -> mon_types (mon_type));
diesel::joinable!(mon_type_mapping -> mons (mon_id));
diesel::joinable!(move_flags -> moves (move_id));
diesel::joinable!(moves -> mon_types (move_type));
diesel::joinable!(series -> tournaments (tournament_id));
diesel::joinable!(series_trainers -> series (series_id));
diesel::joinable!(series_trainers -> trainers (trainer_id));
diesel::joinable!(trainer_mon_moves -> moves (move_id));
diesel::joinable!(trainer_mon_moves -> trainer_mons (trainer_mon_id));
diesel::joinable!(trainer_mon_stats -> battles (battle_id));
diesel::joinable!(trainer_mon_stats -> mons (mon_id));
diesel::joinable!(trainer_mon_stats -> trainer_mons (trainer_mon_id));
diesel::joinable!(trainer_mon_stats -> trainers (trainer_id));
diesel::joinable!(trainer_mon_stats_moves -> moves (move_id));
diesel::joinable!(trainer_mons -> mons (mon_id));
diesel::joinable!(trainer_mons -> trainers (trainer_id));

diesel::allow_tables_to_appear_in_same_query!(
    battles,
    mon_abilities,
    mon_level_up_moves,
    mon_type_mapping,
    mon_types,
    mons,
    move_flags,
    moves,
    series,
    series_trainers,
    tournaments,
    trainer_mon_moves,
    trainer_mon_stats,
    trainer_mon_stats_moves,
    trainer_mons,
    trainers,
);

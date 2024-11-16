CREATE TABLE trainers (
    id INTEGER PRIMARY KEY NOT NULL
);

CREATE TABLE mon_types (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE mons (
    id INTEGER PRIMARY KEY NOT NULL,
    id_name TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE mon_type_mapping (
    mon_id INTEGER REFERENCES mons(id) NOT NULL,
    mon_type TEXT REFERENCES mon_types(name) NOT NULL
);

CREATE TABLE mon_abilities (
    mon_id INTEGER REFERENCES mons(id) NOT NULL,
    ability TEXT NOT NULL,
    PRIMARY KEY (mon_id, ability)
);

CREATE TABLE moves (
    id INTEGER PRIMARY KEY NOT NULL,
    id_name TEXT NOT NULL,
    name TEXT NOT NULL,
    effect TEXT NOT NULL,
    power INTEGER NOT NULL,
    move_type TEXT REFERENCES mon_types(name) NOT NULL,    
    accuracy INTEGER NOT NULL,
    pp INTEGER NOT NULL,
    secondary_effect_chance INTEGER NOT NULL,
    target TEXT NOT NULL,
    priority INTEGER NOT NULL
);

CREATE TABLE move_flags (
    move_id INTEGER REFERENCES moves(id) NOT NULL,
    flag TEXT NOT NULL,
    PRIMARY KEY (move_id, flag)
);

CREATE TABLE mon_level_up_moves (
    mon_id INTEGER REFERENCES mons(id) NOT NULL,
    move_id INTEGER REFERENCES moves(id) NOT NULL,
    lvl INTEGER NOT NULL,
    PRIMARY KEY (mon_id, move_id, lvl)
);

CREATE TABLE trainer_mons (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    trainer_id INTEGER REFERENCES trainers(id) NOT NULL,
    party_index INTEGER NOT NULL,
    iv INTEGER NOT NULL,
    lvl INTEGER NOT NULL,
    mon_id INTEGER REFERENCES mons(id) NOT NULL,
    held_item TEXT
);

CREATE TABLE trainer_mon_moves (
    trainer_mon_id INTEGER REFERENCES trainer_mons(id) NOT NULL,
    move_id INTEGER REFERENCES moves(id) NOT NULL,
    PRIMARY KEY (trainer_mon_id, move_id)
);

CREATE TABLE tournaments (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    tournament_type TEXT NOT NULL
);

CREATE TABLE series (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tournament_id INTEGER REFERENCES tournaments(id) NOT NULL,
    first_to INTEGER NOT NULL,
    priority INTEGER NOT NULL,
    completed_at BIGINT
);

CREATE TABLE series_trainers (
    series_id INTEGER REFERENCES series(id) NOT NULL,
    trainer_id INTEGER REFERENCES trainers(id) NOT NULL,
    PRIMARY KEY (series_id, trainer_id)
);

CREATE TABLE battles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    series_id INTEGER REFERENCES series(id) NOT NULL,
    created_at BIGINT NOT NULL,
    player_perspective INTEGER REFERENCES trainers(id) NOT NULL,
    opponent_perspective INTEGER REFERENCES trainers(id) NOT NULL,
    player_perspective_won BOOLEAN NOT NULL,
    seed INTEGER NOT NULL,
    video_path TEXT NOT NULL,
    duration INTEGER NOT NULL,
    events TEXT NOT NULL
);

CREATE TABLE trainer_mon_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- I really don't need this but it makes the queries easier
    trainer_id INTEGER REFERENCES trainers(id) NOT NULL,
    trainer_mon_id INTEGER REFERENCES trainer_mons(id) NOT NULL,
    battle_id INTEGER REFERENCES battles(id) NOT NULL,
    -- I really don't need this but it makes the queries easier
    mon_id INTEGER REFERENCES mons(id) NOT NULL,
    damage_dealt INTEGER NOT NULL,
    damage_taken INTEGER NOT NULL,
    times_released INTEGER NOT NULL,
    murders INTEGER NOT NULL,
    deaths INTEGER NOT NULL
);

CREATE TABLE trainer_mon_stats_moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    trainer_mon_stats_id INTEGER REFERENCES trainer_mon_stats(id) NOT NULL,
    trainer_mon_target_id INTEGER REFERENCES trainer_mon_stats(id) NOT NULL,
    move_id INTEGER REFERENCES moves(id) NOT NULL,
    times_used INTEGER NOT NULL,
    damage_dealt INTEGER NOT NULL,
    murders INTEGER NOT NULL
);
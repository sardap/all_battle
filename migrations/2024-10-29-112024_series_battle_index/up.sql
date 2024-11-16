-- Your SQL goes here
CREATE INDEX battles_series_index ON battles(series_id);

-- CREATE INDEX battles_player_perspective_index ON battles(player_perspective);
-- CREATE INDEX battles_opponent_perspective_index ON battles(opponent_perspective);

CREATE INDEX trainer_mon_stats_battle_index ON trainer_mon_stats(battle_id);
CREATE INDEX trainer_mon_stats_trainer_index ON trainer_mon_stats(trainer_id);

CREATE INDEX trainer_mon_stats_moves_trainer_mon_stats_index ON trainer_mon_stats_moves(trainer_mon_stats_id);

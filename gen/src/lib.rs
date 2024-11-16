use all_battle_core::{models, schema};
use diesel::prelude::*;
use diesel::SqliteConnection;
use log::info;
use rand::prelude::SliceRandom;
use runner::Runners;
use std::{
    collections::HashSet,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub mod matchups;
pub mod runner;

pub struct RoundRunnerOptions {
    pub worker_count: usize,
    pub emu_path: std::path::PathBuf,
    pub rom_path: std::path::PathBuf,
    pub bios_path: std::path::PathBuf,
    pub video_path: std::path::PathBuf,
}

pub fn run(options: RoundRunnerOptions, conn: Arc<Mutex<SqliteConnection>>) {
    std::thread::spawn(move || {
        round_runner(options, conn.clone());
    });
}

fn round_runner(options: RoundRunnerOptions, conn: Arc<Mutex<SqliteConnection>>) {
    match std::fs::remove_dir_all("video_work") {
        Ok(_) => {}
        Err(_) => {}
    }
    std::fs::create_dir("video_work").unwrap();

    let pending_series: Vec<i32> = {
        let mut lock = conn.lock().unwrap();
        let conn = lock.deref_mut();

        setup_trainers(conn);

        // let trainer_count = schema::trainers::table
        //     .count()
        //     .get_result::<i64>(conn)
        //     .unwrap();

        let pending_series = get_pending_series(conn);
        // if pending_series.is_empty() {
        //     create_round_robbin_tournaments(conn, "Seeding Round", trainer_count as usize, 2);
        // }

        pending_series
    };

    if pending_series.is_empty() {
        info!("No pending series to run");
        return;
    }

    let (runners, result_rx) = Runners::new(
        options.worker_count,
        conn.clone(),
        options.emu_path.to_str().unwrap(),
        options.rom_path.to_str().unwrap(),
        options.bios_path.to_str().unwrap(),
        options.video_path.clone(),
    );

    std::thread::spawn(move || {
        start_pending_series(conn, runners, pending_series);
    });

    loop {
        let completed_series = match result_rx.try_recv() {
            Ok(series) => series,
            Err(err) => {
                if err != std::sync::mpsc::TryRecvError::Empty {}
                continue;
            }
        };
        info!("Completed series {}", completed_series);
    }
}

pub fn setup_trainers(conn: &mut SqliteConnection) {
    use schema::trainers;

    if trainers::table.count().get_result::<i64>(conn).unwrap() > 0 {
        info!("Trainers already setup skipping");
        return;
    }

    let mut trainers = vec![];
    for i in 1..=854 {
        trainers.push(models::Trainer { id: i });
    }
    diesel::insert_into(trainers::table)
        .values(&trainers)
        .execute(conn)
        .expect("Error saving new trainers");
}

fn get_pending_series(conn: &mut SqliteConnection) -> Vec<i32> {
    use schema::series;

    let series: Vec<_> = series::table
        .select(series::id)
        .filter(series::completed_at.is_null())
        .order_by(series::priority.desc())
        .load::<i32>(conn)
        .expect("Error loading series");

    series
}

#[allow(dead_code)]
fn create_round_robbin_tournaments(
    conn: &mut SqliteConnection,
    name: &str,
    group_size: usize,
    first_to: i32,
) {
    use schema::series;
    use schema::series_trainers;
    use schema::tournaments;
    use schema::trainers;

    info!(
        "Creating a round robbin tournament with group size {}",
        group_size
    );

    let mut rng = rand::thread_rng();

    let mut trainers: Vec<models::Trainer> = trainers::table
        .load::<models::Trainer>(conn)
        .expect("Error loading trainers");

    /*
        N*(N-1)/2
        FRLG + Emerald = 1494 = 1494*(1494-1)/2 = 1,115,271
        Emerald = 854 = 854*(854-1)/2 = 365,231

        N X (N-1)/2 * (854 / N)
        7 * 6/2 * (854 / 7) = 2562
        14 * 13/2 * (854 / 14) = 5551
        61 * 60/2 * (854 / 61) = 25620
        122 * 121/2 * (854 / 122) = 51667
    */
    trainers.shuffle(&mut rng);

    let group_count = trainers.len() / group_size;
    let mut groups = Vec::<Vec<&models::Trainer>>::new();
    let mut group_index = 0;
    for _ in 0..group_count {
        groups.push(vec![]);
    }
    for trainer in &trainers {
        groups[group_index].push(trainer);
        group_index += 1;
        if group_index >= group_count {
            group_index = 0;
        }
    }

    let max_group_size = groups.iter().map(|g| g.len()).max().unwrap();
    let number_of_max_groups = groups.iter().filter(|g| g.len() == max_group_size).count();
    let min_group_size = groups.iter().map(|g| g.len()).min().unwrap();
    let number_of_min_groups = groups.iter().filter(|g| g.len() == min_group_size).count();

    info!(
        "{} Trainers will need {} groups with {:2} per group ({}({}), {}({})) with ({},{}) matches per group",
        trainers.len(),
        group_count,
        trainers.len() as f32 / group_count as f32,
        number_of_min_groups,
        min_group_size,
        number_of_max_groups,
        max_group_size,
        (min_group_size * (min_group_size - 1) / 2),
        (max_group_size * (max_group_size - 1) / 2)
    );

    for (i, group) in groups.iter().enumerate() {
        let tournament = diesel::insert_into(tournaments::table)
            .values(&models::NewTournament {
                name: format!("{} Group {}", name, i + 1),
                tournament_type: "round_robbin".to_string(),
            })
            .get_result::<models::Tournament>(conn)
            .expect("Error saving new tournament");

        info!("Creating tournament {} ", tournament.name);

        let mut pairs = HashSet::new();
        for i in group {
            for j in group {
                if i.id == j.id {
                    continue;
                }
                let pair = if i.id < j.id {
                    (i.id, j.id)
                } else {
                    (j.id, i.id)
                };
                pairs.insert(pair);
            }
        }

        let mut pairs = pairs.into_iter().collect::<Vec<_>>();
        pairs.shuffle(&mut rng);

        info!("Creating {} series", pairs.len());

        let mut pending_series = vec![];
        let mut series_trainers = vec![];

        for (j, (a, b)) in pairs.into_iter().enumerate() {
            let priority = (i * 100000) + j;

            pending_series.push(models::NewSeries {
                first_to,
                priority: priority as i32,
                tournament_id: tournament.id,
            });

            // This is a Hack but series IDS are sequential so we can use the index
            series_trainers.push(models::SeriesTrainer {
                series_id: j as i32 + 1,
                trainer_id: a,
            });
            series_trainers.push(models::SeriesTrainer {
                series_id: j as i32 + 1,
                trainer_id: b,
            });
        }

        diesel::insert_into(series::table)
            .values(&pending_series)
            .execute(conn)
            .expect("Error saving new series");

        diesel::insert_into(series_trainers::table)
            .values(&series_trainers)
            .execute(conn)
            .expect("Error saving new series trainer");
    }

    info!("Created tournaments");
}

fn start_pending_series(
    conn: Arc<Mutex<SqliteConnection>>,
    mut runners: Runners,
    mut pending_series: Vec<i32>,
) {
    loop {
        let series_id = match pending_series.pop() {
            Some(series_id) => series_id,
            None => {
                pending_series = get_pending_series(&mut conn.lock().unwrap());
                if pending_series.is_empty() {
                    break;
                }
                pending_series.pop().unwrap()
            }
        };

        runners.run(series_id);
    }
}

use log::info;

use all_battle_core::gen3::populate_gen3_tables;
use diesel::{Connection, SqliteConnection};
use lazy_static::lazy_static;
use options::OPTIONS;
use simple_logger::SimpleLogger;
#[cfg(feature = "gen")]
use std::sync::{Arc, Mutex};

mod options;

lazy_static! {
    static ref DATABASE_URL: String =
        format!("sqlite://{}", OPTIONS.database_path.to_str().unwrap());
}

#[cfg(feature = "web")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    use all_battle_web::{get_connection_pool, AppState, TrainerMonCache, TrainerRankCache, MonStatsCache};

    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    info!("Starting web");

    #[cfg(feature = "gen")]
    {
        let conn = Arc::new(Mutex::new(establish_connection(&DATABASE_URL)));
        start_runner(conn.clone());
    }

    let trainer_mon_cache = TrainerMonCache::new();
    let trainer_rank_cache = TrainerRankCache::new();
    let mon_stats_cache = MonStatsCache::new();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(
                get_connection_pool(&DATABASE_URL),
                OPTIONS.web_video_path.clone(),
                OPTIONS.build_dir.clone(),
                trainer_mon_cache.clone(),
                trainer_rank_cache.clone(),
                mon_stats_cache.clone(),
            )))
            .service(all_battle_web::routes())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(not(feature = "web"))]
fn main() {
    use std::time::Duration;

    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    info!("Starting non web");

    #[cfg(feature = "gen")]
    {
        let conn = Arc::new(Mutex::new(establish_connection(&DATABASE_URL)));

        start_runner(conn.clone());
    }
    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(feature = "gen")]
fn start_runner(conn: Arc<Mutex<SqliteConnection>>) {
    use all_battle_gen::RoundRunnerOptions;

    info!("Starting runner");

    let conn = conn.clone();
    all_battle_gen::run(
        RoundRunnerOptions {
            worker_count: OPTIONS.worker_count,
            emu_path: OPTIONS.emu_path.clone(),
            rom_path: OPTIONS.rom_path.clone(),
            bios_path: OPTIONS.bios_path.clone(),
            video_path: OPTIONS.video_path.clone(),
        },
        conn.clone(),
    );
}

pub fn establish_connection(url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(url).unwrap();
    populate_gen3_tables(&mut conn);
    conn
}

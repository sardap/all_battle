use diesel::prelude::*;
use log::debug;
use log::error;
use log::info;

use crate::matchups::complete_series;
use std::panic;
use std::path::PathBuf;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::sync::Mutex;
use std::{
    io::{BufRead, BufReader},
    net::{Ipv4Addr, UdpSocket},
    process::{Command, Stdio},
    sync::mpsc::{sync_channel, Receiver},
};

pub struct Runners {
    runners_tx: Vec<SyncSender<i32>>,
    last_runner_index: usize,
}

impl Runners {
    pub fn new(
        worker_count: usize,
        conn: Arc<Mutex<SqliteConnection>>,
        emu_path: &str,
        rom_path: &str,
        bios_path: &str,
        video_path: PathBuf,
    ) -> (Runners, Receiver<i32>) {
        let (result_tx, result_rx) = sync_channel(1000);

        let mut runners_tx = vec![];
        for i in 0..worker_count {
            let result_tx = result_tx.clone();
            let (tx, rx) = sync_channel(10000);
            runners_tx.push(tx);
            let conn = conn.clone();
            let emu_path = emu_path.to_string();
            let rom_path = rom_path.to_string();
            let bios_path = bios_path.to_string();
            let video_path = video_path.clone();
            std::thread::spawn(move || {
                let mut game = Game::new(i, &emu_path, &rom_path, &bios_path, video_path.clone());
                loop {
                    let series: i32 = rx.recv().unwrap();

                    info!("{} Running series {}", game.id, series);

                    let result = panic::catch_unwind(|| {
                        complete_series(conn.clone(), &game, series);

                        debug!("{} Sending {} series completed", game.id, series);
                        result_tx.send(series).unwrap();
                    });

                    if let Err(err) = result {
                        game = Game::new(i, &emu_path, &rom_path, &bios_path, video_path.clone());
                        error!("Panic in runner {:?}", err);
                    }
                }
            });
        }

        (
            Runners {
                runners_tx,
                last_runner_index: 0,
            },
            result_rx,
        )
    }

    fn rotate_runner(&mut self) {
        self.last_runner_index = (self.last_runner_index + 1) % self.runners_tx.len();
    }

    pub fn run(&mut self, series: i32) {
        self.runners_tx[self.last_runner_index]
            .send(series.clone())
            .unwrap();
        self.rotate_runner();
    }
}

pub struct Game {
    _process: std::process::Child,
    socket: UdpSocket,
    gba_address: String,
    pub id: usize,
    pub video_path: PathBuf,
    pub rng_address: u32,
    pub battler_address: u32,
    pub player_address: u32,
    pub out: Receiver<String>,
}

impl Game {
    fn new(
        id: usize,
        emu_path: &str,
        rom_path: &str,
        bios_path: &str,
        video_path: PathBuf,
    ) -> Game {
        info!("{} Starting emulator", id);

        let mut child = Command::new(emu_path)
            .arg(rom_path)
            .arg("--skip-bios")
            .arg("--bios")
            .arg(bios_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start emulator");

        let (tx, rx) = sync_channel(100);

        let stderr = child.stderr.take().unwrap();

        std::thread::spawn(move || {
            let lines = BufReader::new(stderr).lines();
            for line in lines {
                match line {
                    Ok(line) => {
                        // println!("{}", line);
                        match tx.send(line) {
                            Ok(_) => {}
                            Err(_) => break,
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();

        let mut gba_address: Option<String> = None;

        let mut rng_address: Option<u32> = None;

        let mut battler_address: Option<u32> = None;

        let mut player_address: Option<u32> = None;

        loop {
            let line = rx.recv().unwrap();
            if line.contains("Open UDP socket on ") {
                let address = line
                    .split("Open UDP socket on ")
                    .last()
                    .unwrap()
                    .to_string();
                gba_address = Some(address);
            } else if line.contains("PlayerAddress") {
                let splits: Vec<_> = line.split(" ").collect();
                let mem_address: u32 = splits[splits.len() - 2].parse::<u32>().unwrap();
                player_address = Some(mem_address);
            } else if line.contains("BattlerAddress") {
                let splits: Vec<_> = line.split(" ").collect();
                let mem_address: u32 = splits[splits.len() - 2].parse::<u32>().unwrap();
                battler_address = Some(mem_address);
            } else if line.contains("RngValueAddress") {
                let splits: Vec<_> = line.split(" ").collect();
                let mem_address: u32 = splits[splits.len() - 2].parse::<u32>().unwrap();
                rng_address = Some(mem_address);
            }

            if gba_address.is_some()
                && rng_address.is_some()
                && battler_address.is_some()
                && player_address.is_some()
            {
                break;
            }
        }
        info!("{} Emulator started", id);

        Game {
            _process: child,
            socket,
            id,
            gba_address: gba_address.unwrap(),
            video_path,
            rng_address: rng_address.unwrap(),
            battler_address: battler_address.unwrap(),
            player_address: player_address.unwrap(),
            out: rx,
        }
    }

    pub fn send_key_down(&self, button: &str) {
        self.socket
            .send_to(format!("key_down,{}", button).as_bytes(), &self.gba_address)
            .expect("Fuck");
    }

    pub fn send_key_up(&self, button: &str) {
        self.socket
            .send_to(format!("key_up,{}", button).as_bytes(), &self.gba_address)
            .expect("Fuck");
    }

    pub fn send_write_u16(&self, mem_address: u32, value: u16) {
        self.socket
            .send_to(
                format!("set_u16,{},{}", mem_address, value).as_bytes(),
                &self.gba_address,
            )
            .expect("broken");
    }

    pub fn send_start_recording(&self) {
        self.socket
            .send_to(format!("start_recording").as_bytes(), &self.gba_address)
            .expect("broken");
    }

    pub fn send_stop_recording(&self, file_name: &str) {
        self.socket
            .send_to(
                format!("stop_recording,{}", file_name).as_bytes(),
                &self.gba_address,
            )
            .expect("broken");
    }

    pub fn send_display_enabled(&self, enabled: bool) {
        self.socket
            .send_to(
                format!("display_enabled,{}", enabled).as_bytes(),
                &self.gba_address,
            )
            .expect("broken");
    }
}

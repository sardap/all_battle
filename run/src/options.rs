use std::path::PathBuf;

use lazy_static::lazy_static;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "all-battle-core")]
pub struct Options {
    #[structopt(name = "database-path", parse(from_os_str))]
    pub database_path: PathBuf,

    #[cfg(feature = "gen")]
    #[structopt(name = "video-path", parse(from_os_str))]
    pub video_path: PathBuf,

    #[cfg(feature = "gen")]
    #[structopt(name = "emu-path", parse(from_os_str))]
    pub emu_path: PathBuf,

    #[cfg(feature = "gen")]
    #[structopt(name = "rom-path", parse(from_os_str))]
    pub rom_path: PathBuf,

    #[cfg(feature = "gen")]
    #[structopt(name = "bios-path", parse(from_os_str))]
    pub bios_path: PathBuf,

    #[cfg(feature = "gen")]
    #[structopt(name = "worker-count")]
    pub worker_count: usize,

    #[cfg(feature = "web")]
    #[structopt(name = "web-video-path", parse(from_os_str))]
    pub web_video_path: PathBuf,

    #[cfg(feature = "web")]
    #[structopt(name = "build-dir", parse(from_os_str))]
    pub build_dir: PathBuf,
}

lazy_static! {
    pub static ref OPTIONS: Options = Options::from_args();
}

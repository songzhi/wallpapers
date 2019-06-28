#[macro_use]
mod singleton;
mod daemon;
pub mod config;

use wallpaper;
use std::thread::sleep;
use crate::daemon::DaemonRunner;
use daemon::State;
use std::sync::mpsc::Receiver;

pub fn set_random_wallpaper(collections_param: Option<String>) {
    static DEFAULT_URL: &str = "https://source.unsplash.com/random?orientation=landscape";

    if let Some(collections) = collections_param {
        let url = format!("{}&collections={}", DEFAULT_URL, collections);
        wallpaper::set_from_url(&url);
    } else {
        wallpaper::set_from_url(DEFAULT_URL);
    };
}

pub fn main_loop() {
    loop {
        let config = config::get_config();
        let config = config.lock().expect("");
        let collections_param = if config.collections.is_empty() {
            None
        } else {
            Some(config.collections.join(","))
        };
        set_random_wallpaper(collections_param);
        let duration = config.get_duration();
        sleep(duration);
    }
}

pub fn run() {
    let daemon = daemon::Daemon {
        name: "wallpapers_rs".to_string()
    };
    daemon.run(|rx: Receiver<State>| main_loop());
}
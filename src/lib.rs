mod daemon;
pub mod config;

pub use config::*;


use std::thread::sleep;
use crate::daemon::DaemonRunner;
use daemon::State;
use std::sync::mpsc::Receiver;
use crate::config::Config;


pub fn set_random_wallpaper(config: &Config) {
    // just unwrap because we use Config::default()
    let mut url = format!("https://source.unsplash.com/random/{}/?{}&orientation={}&featured=1",
                          config.image_resolution.as_ref().unwrap(),
                          config.search_keyword.as_ref().unwrap(),
                          config.orientation.unwrap());
    if let Some(ref collections) = config.collections {
        let collections = collections.join(",");
        url.push_str(&format!("&collections={}", collections));
    };
    wallpaper::set_from_url(&url);
}

pub fn main_loop() {
    loop {
        let config = config::get_config();
        let config = config.lock().expect("Lock config failed");
        set_random_wallpaper(&config);
        let duration = config.get_interval();
        sleep(duration);
    }
}

pub fn run_as_daemon() {
    let daemon = daemon::Daemon {
        name: "main".to_string()
    };
    daemon.run(|_: Receiver<State>| main_loop());
}
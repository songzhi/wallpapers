use std::time::Duration;
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use std::env;
use std::path::PathBuf;
use std::fs::OpenOptions;
use fs2::FileExt;
use toml;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub collections: Option<Vec<String>>,
    interval: Option<u64>, // in minutes
}

static CONFIG: OnceCell<Arc<Mutex<Config>>> = OnceCell::new();

const DEFAULT_INTERVAL: u64 = 10;

pub fn get_config() -> Arc<Mutex<Config>> {
    CONFIG.get_or_init(|| {
        let config_path = env::current_exe().unwrap().with_file_name("wallpapers.toml");
        let mut config_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(config_path.as_path()).expect("Open config file failed");
        config_file.try_lock_exclusive().expect("Other program running"); // unwrap on purpose
        let mut buf = String::new();
        config_file.read_to_string(&mut buf).expect("Can't read config file content to string");
        let config: Config = toml::from_str(&buf).unwrap_or(Config::default());

        Arc::new(Mutex::new(config))
    }).clone()
}

impl Config {
    #[inline]
    pub fn get_interval(&self) -> Duration {
        Duration::from_secs(self.interval.unwrap_or(DEFAULT_INTERVAL) * 60)
    }
    #[inline]
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = Some(interval.as_secs() / 60);
    }
    #[inline]
    pub fn update(&mut self, config: Config) {
        *self = config;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            collections: None,
            interval: Some(DEFAULT_INTERVAL),
        }
    }
}
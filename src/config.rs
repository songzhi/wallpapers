use std::time::Duration;
use std::sync::{Mutex, Arc};
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs::OpenOptions;
use fs2::FileExt;
use toml;
use std::io::Read;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub collections: Option<Vec<String>>,
    // in minutes
    interval: Option<u64>,
    pub orientation: Option<Orientation>,
    // based on orientation
    pub image_resolution: Option<String>,
    // "wallpapers",etc.
    pub search_keyword: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}

impl Orientation {
    pub fn get_image_resolution(&self) -> &'static str {
        match self {
            Orientation::Landscape => "1920x1080",
            Orientation::Portrait => "1080x1920",
            Orientation::Squarish => "1440x1440"
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", toml::to_string(self)
            .expect("Serialize Orientation failed").to_ascii_lowercase())
    }
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
        let config = toml::from_str::<Config>(&buf).expect("Can't parse config");
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
}

impl Default for Config {
    fn default() -> Self {
        let orientation = Orientation::Landscape;
        Self {
            collections: None,
            interval: Some(DEFAULT_INTERVAL),
            orientation: Some(orientation),
            image_resolution: Some(orientation.get_image_resolution().to_string()),
            search_keyword: Some("wallpapers".to_string()),
        }
    }
}
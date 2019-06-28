use std::time::Duration;
use std::sync::{MutexGuard, Mutex, Arc};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Config {
    pub collections: Vec<String>,
    duration: u64, // in minutes
}

static CONFIG: OnceCell<Arc<Mutex<Config>>> = OnceCell::new();

pub fn get_config() -> Arc<Mutex<Config>> {
    CONFIG.get_or_init(|| {
        Arc::new(Mutex::new(Config::default()))
    }).clone()
}

impl Config {
    #[inline]
    pub fn get_duration(&self) -> Duration {
        Duration::from_secs(self.duration * 60)
    }
    #[inline]
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration.as_secs() / 60;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            collections: vec!["desktop-wallpapers".to_string(),
                              "macos-desktop-wallpapers".to_string(),
            ],
            duration: 15,
        }
    }
}
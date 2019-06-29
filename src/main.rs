#![windows_subsystem = "windows"]

use wallpapers::run_as_daemon;
use toml;
use wallpapers::Orientation;
fn main() {
//    let x = toml::from_str::<Orientation>("landscape").unwrap();
    let y = toml::to_string(&Orientation::Landscape).unwrap();
    println!("{:?}", y);
}

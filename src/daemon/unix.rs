use super::*;

use daemonize::Daemonize;
use std::process::exit;

impl DaemonRunner for Daemon {
    fn run<F: 'static + FnOnce(Receiver<State>)>(&self, f: F) -> Result<(), Error> {
        let d = Daemonize::new()
            .pid_file("/data/wallpapers/wallpapers.pid")
            .chown_pid_file(true);
        match d.start() {
            Ok(_) => f(),
            Err(_) => exit(-1),
        };
        Ok(())
    }
}
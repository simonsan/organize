mod file;
mod cli;
mod config;

use regex::Regex;
use std::path::Path;
use crate::config::Config;
use std::sync::mpsc::channel;
use notify::{raw_watcher, RecursiveMode, RawEvent, Watcher};
use std::thread;
use std::time::Duration;


fn main() -> std::io::Result<()> {
    let config = Config::new()?;
    let ext_to_rule = config.map_extensions_to_rules();
    let (tw, rx) = channel();
    let mut watcher = raw_watcher(tw).unwrap();
    watcher.watch(&config.watch, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                   path: Some(abs_path),
                   op: Ok(op),
                   cookie: _,
               }) => match op {
                notify::op::CREATE => {
                    if abs_path.is_file() && ext_to_rule.keys().any(|x| x == &abs_path.extension().unwrap().to_str().unwrap()) {
                        // if the current file extension exists in the config
                        let rule = ext_to_rule.get(&abs_path.extension().unwrap().to_str().unwrap().to_string()).unwrap();
                        let file = file::File::from(&abs_path);

                        if rule.has_valid_detailed_rules() {
                            if file.matches_detailed_rules(rule) {
                                thread::sleep(Duration::from_millis(5000));
                                file.rename(rule)?;
                            }
                            continue
                        }
                        thread::sleep(Duration::from_millis(5000));
                        file.rename(rule)?;
                    }
                }
                _ => continue,
            },
            Ok(event) => eprintln!("broken event: {:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

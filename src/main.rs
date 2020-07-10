mod cli;
mod config;
mod file;

use crate::config::{Config, Rule};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use regex::Regex;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::io::Error;

fn main() -> std::io::Result<()> {
    let config = Config::new()?;
    let ext_to_rule = config.map_extension_to_rule();
    let (tw, rx) = channel();
    let mut watcher = raw_watcher(tw).unwrap();
    watcher
        .watch(&config.watch, RecursiveMode::Recursive)
        .unwrap();

    'outer: loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(abs_path),
                op: Ok(op),
                cookie: _,
            }) => match op {
                notify::op::CREATE => {
                    if abs_path.is_file()
                        && ext_to_rule
                            .keys()
                            .any(|x| x == &abs_path.extension().unwrap().to_str().unwrap())
                    {
                        let extension = &abs_path.extension().unwrap().to_str().unwrap().to_string();
                        if ext_to_rule.contains_key(extension) {
                            let rule = ext_to_rule.get(extension).unwrap();
                            if rule.patterns.is_some() {
                                for pattern in rule.patterns.as_ref().unwrap().iter() {
                                    let regex = Regex::new(&pattern.regex)
                                        .expect("ERROR: invalid regex");
                                    if regex.is_match(abs_path.to_str().unwrap()) {
                                        thread::sleep(Duration::from_millis(5000));
                                        file::File::from(&abs_path).rename(&pattern.dst);
                                        continue 'outer;
                                    }
                                }
                            }
                            thread::sleep(Duration::from_millis(5000));
                            file::File::from(&abs_path).rename(&rule.dst);
                        }
                    }
                }
                _ => continue,
            },
            Ok(event) => eprintln!("broken event: {:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

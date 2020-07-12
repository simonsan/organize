use crate::config::actions::file::File;
use crate::config::{Config, Rule};
use notify::op;
use notify::{raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub struct Notifier {
    watcher: RecommendedWatcher,
    receiver: Receiver<RawEvent>,
}

impl Notifier {
    pub fn new() -> Notifier {
        let (sender, receiver) = channel();
        let watcher = raw_watcher(sender).unwrap();

        Notifier { watcher, receiver }
    }

    pub fn watch(&mut self, user_config: Config) {
        for path in user_config.args.watch.iter() {
            self.watcher
                .watch(path, RecursiveMode::NonRecursive)
                .unwrap();
        }

        loop {
            match self.receiver.recv() {
                Ok(RawEvent {
                    path: Some(abs_path),
                    op: Ok(op),
                    cookie: _,
                }) => match op {
                    op::CREATE => {
                        let extension = abs_path.extension();
                        if abs_path.is_file()
                            && extension.is_some()
                            && extension.unwrap().to_str().is_some()
                        {
                            let fields =
                                user_config.rules.get(extension.unwrap().to_str().unwrap()); // safe unwraps
                            if fields.is_some() {
                                let rule = Rule::from_fields(fields.unwrap()); // safe unwrap
                                let file = File::from(&abs_path);
                                let dst = rule.get_file_dst(&file);
                                if user_config.args.delay > 0 {
                                    thread::sleep(Duration::from_millis(
                                        (user_config.args.delay as u64) * 1000,
                                    ));
                                }
                                match file.rename(dst) {
                                    Ok(_) => continue,
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        continue;
                                    }
                                }
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
}

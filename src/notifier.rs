use crate::config::{Rule, UserConfig};
use crate::file::File;
use notify::{raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher};
use notify::op;
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

    pub fn watch(&mut self, user_config: UserConfig) {
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
                        if abs_path.is_file() {
                            let extension = abs_path.extension().unwrap().to_str().unwrap();
                            let rule = Rule::from_yaml(&user_config.rules[extension]);
                            let file = File::from(&abs_path);
                            if !(rule.is_null() || rule.is_badvalue()) {
                                let dst = rule.get_dst_for_file(&file);
                                if user_config.args.delay > 0 {
                                    thread::sleep(Duration::from_millis((user_config.args.delay as u64) * 1000));
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
                    },
                    _ => continue,
                },
                Ok(event) => eprintln!("broken event: {:?}", event),
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    }
}

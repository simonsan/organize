use crate::{
    configuration::{
        actions::process_actions,
        folder2rules,
        options::{
            Options,
            TemporaryOptions,
        },
        Rule,
    },
    file::File,
    init,
    subcommands::edit::TemporaryRules,
};
use notify::{
    op,
    raw_watcher,
    RawEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher as OtherWatcher,
};
use std::{
    path::PathBuf,
    sync::mpsc::{
        channel,
        Receiver,
    },
};

pub struct Watcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<RawEvent>,
}

impl Default for Watcher {
    fn default() -> Self {
        let (sender, receiver) = channel();
        let watcher = raw_watcher(sender).unwrap();
        Watcher {
            watcher,
            receiver,
        }
    }
}

impl Watcher {
    pub fn new() -> Self {
        Watcher::default()
    }

    pub fn watch(&mut self, rules: &[Rule], config_file: PathBuf) {
        self.watcher.watch(&config_file, RecursiveMode::NonRecursive).unwrap();
        for rule in rules.iter() {
            for folder in rule.folders.iter() {
                let is_recursive = if folder.options.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                self.watcher.watch(&folder.path, is_recursive).unwrap();
            }
        }
        let folder2rules = folder2rules(&rules);

        // THERE CAN ONLY BE ONE WATCHER, WHICH CAN WATCH MULTIPLE FOLDERS
        // create a folder2rule hash table to map folders to their corresponding rules
        // and maybe a path2folder hash table, where folder is the custom struct we defined
        loop {
            if let Ok(RawEvent {
                path: Some(abs_path),
                op: Ok(op),
                ..
            }) = self.receiver.recv()
            {
                if let op::CREATE = op {
                    let file = File::from(&abs_path);
                    if let Ok(mut file) = file {
                        if file.path.is_file() {
                            let parent_dir = file.path.parent().unwrap().to_path_buf();
                            let values = folder2rules.get(&parent_dir).unwrap().to_owned();
                            for (rule, i) in values {
                                if rule.options.ignore.contains(&parent_dir) {
                                    continue;
                                }
                                let folder = rule.folders.get(i).unwrap();
                                let Options {
                                    watch, ..
                                } = folder.options;

                                if watch && file.matches_filters(&rule.filters) {
                                    println!("{}", &abs_path.display());
                                    process_actions(&rule.actions, &mut file).unwrap();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
use crate::{
    configuration::{
        actions::{
            process_actions,
            Actions,
        },
        filters::Filters,
        folder2rule,
        folders::Folder,
        options::Options,
        Rule,
    },
    file::File,
    subcommands::config::Rules,
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

    pub fn watch(&mut self, rules: &Rules) {
        for rule in rules.iter() {
            for folder in rule.folders.iter() {
                let is_recursive = if folder.options.as_ref().unwrap().recursive.unwrap() {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                self.watcher.watch(&folder.path, is_recursive).unwrap();
            }
        }
        let folder2rule = folder2rule(&rules);
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
                            let (rule, folder_index) = *folder2rule.get(&parent_dir).unwrap();
                            if rule
                                .options
                                .as_ref()
                                .unwrap()
                                .ignore
                                .as_ref()
                                .unwrap()
                                .contains(&parent_dir)
                            {
                                continue;
                            }
                            let folder = rule.folders.get(folder_index).unwrap();
                            println!("{:?}", folder);
                            let Options {
                                watch, ..
                            } = folder.options.as_ref().unwrap();
                            let filters = rule.filters.as_ref().unwrap();
                            if watch.is_some() && watch.unwrap() && file.matches_filters(filters) {
                                println!("{}", &abs_path.display());
                                process_actions(&rule.actions, &mut file).unwrap();
                            }
                            // 'inner: for folder in &self.folders {
                            //     let Options {
                            //         ignore, ..
                            //     } = self.rule.options.as_ref().unwrap();
                            //     if parent_dir == folder.path
                            //         && !ignore.as_ref().unwrap().contains(&parent_dir.to_path_buf())
                            //     {
                            //         let Options {
                            //             watch, ..
                            //         } = folder.options.as_ref().unwrap();
                            //         let filters = self.rule.filters.as_ref().unwrap();
                            //         if watch.is_some() && watch.unwrap() && file.matches_filters(filters) {
                            //             println!("{}", &abs_path.display());
                            //             process_actions(&self.rule.actions, &mut file).unwrap();
                            //         }
                            //         break 'inner;
                            //     }
                            // }
                        }
                    }
                }
            }
        }
    }
}

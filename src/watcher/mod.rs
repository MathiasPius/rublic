pub mod messages;
mod handlers;

use std::fs::{read_dir};
use std::path::PathBuf;
use std::collections::HashMap;
use actix::{Actor, Context, Addr, Arbiter};
use crate::errors::ServiceError;
use crate::database::DbExecutor;
use crate::inotify::{Inotify, EventMask, WatchMask};

// Listens for filesystem events
pub struct ArchiveWatcher {
    pub children: HashMap<PathBuf, Addr<DomainWatcher>>,
    pub dir: PathBuf,
    pub db: Addr<DbExecutor>
}

pub struct DomainWatcher {
    pub db: Addr<DbExecutor>,
    pub dir: PathBuf
}

impl ArchiveWatcher {
    pub fn new(db: Addr<DbExecutor>, dir: PathBuf) -> Self {
        ArchiveWatcher {
            children: HashMap::new(),
            dir: dir,
            db: db.clone()
        }
    }

    pub fn parse_domains(&mut self) -> Result<(), ServiceError> {
        if let Ok(entries) = read_dir(&self.dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(filetype) = entry.file_type() {
                        if filetype.is_dir() {
                            let watcher = DomainWatcher {
                                db: self.db.clone(),
                                dir: entry.path()
                            };

                            self.children.insert(
                                entry.path(), 
                                Arbiter::start(|_| watcher)
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Actor for ArchiveWatcher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {

        self.parse_domains().unwrap();

        let mut watcher = Inotify::init()
            .expect("failed to initialize Inotify instance");

        watcher
            .add_watch(&self.dir, WatchMask::CREATE | WatchMask::DELETE)
            .expect("failed to start watching directory");

        println!("ArchiveWatcher for {} started!", &self.dir.display());

        let mut buffer = [0u8; 4096];
        loop {
            let events = watcher
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if !event.mask.contains(EventMask::ISDIR) {
                    continue
                }

                if event.mask.contains(EventMask::CREATE) {
                    println!("Domain created: {:?}", event.name);
                    let mut fullpath = self.dir.clone();
                    fullpath.push(event.name.unwrap());

                    let watcher = DomainWatcher {
                        db: self.db.clone(),
                        dir: fullpath.clone()
                    };

                    self.children.insert(
                        fullpath, 
                        Arbiter::start(|_| watcher)
                    );
                } else if event.mask.contains(EventMask::DELETE) {
                    println!("Domain deleted: {:?}", event.name);
                }
            }
        }
    }
}

impl Actor for DomainWatcher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut watcher = Inotify::init()
            .expect("failed to initialize Inotify instance");

        watcher
            .add_watch(&self.dir, WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE | WatchMask::MOVED_FROM)
            .expect("failed to start watching directory");

        println!("DomainWatcher for {} started!", &self.dir.display());

        let mut buffer = [0u8; 4096];
        loop {
            let events = watcher
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                // Ignore all directory effects
                /*if event.mask.contains(EventMask::ISDIR) {
                    continue
                }
                */

                if event.mask.contains(EventMask::CREATE) || event.mask.contains(EventMask::MODIFY) {
                    println!("Domain {:?} modified: {:?}", self.dir.file_name().unwrap(), event.name);
                } else {
                    println!("Domain {:?} deleted: {:?}", self.dir.file_name().unwrap(), event.name);
                }
            }
        }
    }
}
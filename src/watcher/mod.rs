pub mod messages;
mod handlers;

use regex::Regex;
use std::fs::{read_dir};
use std::path::PathBuf;
use std::collections::HashMap;
use futures::Future;
use actix::{Actor, Context, Addr, Arbiter, ActorContext};
use crate::errors::ServiceError;
use crate::database::DbExecutor;
use crate::database::messages::{CreateDomain, DeleteDomain};
use crate::certman::messages::{CertificateDiscovered, CertificateDisappeared};
use crate::certman::CertificateManager;
use crate::inotify::{Inotify, EventMask, WatchMask};

// Listens for filesystem events
pub struct ArchiveWatcher {
    pub db: Addr<DbExecutor>,
    pub certman: Addr<CertificateManager>,
    pub children: HashMap<PathBuf, Addr<DomainWatcher>>,
    pub dir: PathBuf,
}

pub struct DomainWatcher {
    pub db: Addr<DbExecutor>,
    pub certman: Addr<CertificateManager>,
    pub dir: PathBuf,
}

impl ArchiveWatcher {
    pub fn new(db: Addr<DbExecutor>, certman: Addr<CertificateManager>, dir: PathBuf) -> Self {
        ArchiveWatcher {
            children: HashMap::new(),
            dir: dir,
            db: db.clone(),
            certman: certman.clone()
        }
    }

    pub fn watch(&mut self, path: PathBuf) {
        // Spin up a new DomainWatcher for the directory
        let watcher = DomainWatcher {
            db: self.db.clone(),
            certman: self.certman.clone(),
            dir: path.clone()
        };

        self.children.insert(
            path.clone(), 
            Arbiter::start(|_| watcher)
        );

        // Create domain in DB, but ignore if it already exists
        self.db.send(CreateDomain { 
            fqdn: path.file_name().unwrap().to_string_lossy().into() 
        }).flatten().wait().ok();
    }

    pub fn parse_domains(&mut self) -> Result<(), ServiceError> {
        if let Ok(entries) = read_dir(&self.dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(filetype) = entry.file_type() {
                        if filetype.is_dir() {
                            self.watch(entry.path());
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
            .expect("AW: failed to initialize Inotify instance");

        watcher
            .add_watch(&self.dir, WatchMask::CREATE | WatchMask::MOVED_FROM | WatchMask::MOVED_TO | WatchMask::DELETE)
            .expect("AW: failed to start watching directory");

        println!("AW for {} started!", &self.dir.display());

        let mut buffer = [0u8; 4096];
        loop {
            let events = watcher
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if !event.mask.contains(EventMask::ISDIR) {
                    continue
                }

                if event.mask.intersects(EventMask::CREATE | EventMask::MOVED_TO) {
                    if let Some(name) = event.name {
                        println!("AW: domain {:?} was created!", name);
                        let mut fullpath = self.dir.clone();
                        fullpath.push(name);

                        self.watch(fullpath.clone());
                    }
                } else if event.mask.intersects(EventMask::DELETE | EventMask::MOVED_FROM) {
                    if let Some(name) = event.name {
                        let path = PathBuf::from(name);

                        println!("AW: domain {:?} was deleted!", name);
                        if self.children.contains_key(&path) {
                            self.children.remove(&path);
                        }
                    }
                }
            }
        }
    }
}


impl Actor for DomainWatcher {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        lazy_static! {
            static ref CERT_PATTERN: Regex = Regex::new(r"(\w+)([0-9]+)\.(\w+)").unwrap();
        }

        let mut watcher = Inotify::init()
            .expect("DW: failed to initialize Inotify instance");

        watcher
            .add_watch(&self.dir, WatchMask::DELETE_SELF | WatchMask::MOVE_SELF | WatchMask::CREATE | WatchMask::DELETE | WatchMask::MOVED_FROM | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE)
            .expect("DW: failed to start watching directory");

        println!("DW: for {} started!", &self.dir.display());

        let mut buffer = [0u8; 4096];
        loop {
            let events = watcher
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if let Some(name) = event.name {
                    if (EventMask::CLOSE_WRITE | EventMask::MOVED_TO).intersects(event.mask) {
                        println!("DW: domain {:?} modified: {:?} ({:?})", self.dir.file_name().unwrap(), name, event.mask);

                        let mut path = self.dir.clone();
                        path.push(name);

                        self.certman.send(CertificateDiscovered {
                                fqdn: self.dir.file_name().unwrap().to_string_lossy().into(),
                                path: path
                            }).wait().ok();

                    } else if (EventMask::DELETE | EventMask::MOVED_FROM).intersects(event.mask) {
                        println!("DW: domain {:?} deleted: {:?} ({:?})", self.dir.file_name().unwrap(), name, event.mask);

                        let mut path = self.dir.clone();
                        path.push(name);
                        self.certman.send(CertificateDisappeared { path }).wait().ok();
                    }
                }
                
                if (EventMask::DELETE_SELF | EventMask::MOVE_SELF).intersects(event.mask) {
                    println!("DW: domain {:?} deleted! committing seppuku", self.dir.file_name().unwrap());

                    self.db.send(DeleteDomain { 
                        fqdn: self.dir.file_name().unwrap().to_string_lossy().into() 
                    }).flatten().wait().unwrap();

                    ctx.stop();
                    return;
                }
            }
        }
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("DW: watcher for {:?} is kill", self.dir.file_name().unwrap());
    }
}
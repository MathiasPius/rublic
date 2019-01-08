pub mod errors;
pub mod models;

use std::path::PathBuf;
use std::collections::HashMap;
use futures::Future;
use actix::{Actor, Context, Addr, Arbiter};
use crate::database::DbExecutor;
use crate::database::messages::{CreateDomain};
use crate::certman::messages::{CertificateDiscovered, CertificateDisappeared};
use crate::certman::CertificateManager;
use self::models::{FileType, EventType, DirectoryWatcher};

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
            dir,
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
}

impl Actor for ArchiveWatcher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut watcher = DirectoryWatcher::new(self.dir.clone())
            .expect(&format!("unable to launch DirectoryWatcher for {}", self.dir.to_string_lossy()));

        loop {
            if let Ok(event) = watcher.get_event() {
                if event.file_type != FileType::Directory {
                    continue;
                }

                if event.event_type == EventType::Updated {
                    self.watch(event.path);
                } else if event.event_type == EventType::Deleted {
                    if self.children.contains_key(&event.path) {
                        self.children.remove(&event.path);
                    }
                }
            }
        }
    }
}

impl DomainWatcher {
    pub fn discovered_certificate(&mut self, fullpath: PathBuf) {
        self.certman.send(CertificateDiscovered {
            fqdn: self.dir.file_name().unwrap().to_string_lossy().into(),
            path: fullpath
        }).wait().ok();
    }

    pub fn lost_certificate(&mut self, fullpath: PathBuf) {
        self.certman.send(CertificateDisappeared { path: fullpath }).wait().ok();
    }
}

impl Actor for DomainWatcher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {

        let mut watcher = DirectoryWatcher::new(self.dir.clone())
            .expect(&format!("unable to launch DirectoryWatcher for {}", self.dir.to_string_lossy()));

        loop {
            if let Ok(event) = watcher.get_event() {
                if event.file_type != FileType::File {
                    continue;
                }

                if event.event_type == EventType::Updated {
                    self.discovered_certificate(event.path);
                } else if event.event_type == EventType::Deleted {
                    self.lost_certificate(event.path);
                }
            }
        }
    }
}
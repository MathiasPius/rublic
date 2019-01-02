pub mod messages;
mod handlers;

use actix::{Actor, Context, Addr};
use crate::database::DbExecutor;
use crate::inotify::{Inotify, EventMask, WatchMask};

// Listens for filesystem events
pub struct FilesystemWatcher {
    pub dir: String,
    pub db: Addr<DbExecutor>
}

impl FilesystemWatcher {
    pub fn new(db: Addr<DbExecutor>, dir: String) -> Self {
        FilesystemWatcher {
            dir,
            db: db.clone()
        }
    }
}

impl Actor for FilesystemWatcher {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut watcher = Inotify::init()
            .expect("failed to initialize Inotify instance");

        watcher
            .add_watch(
                &self.dir,
                WatchMask::MODIFY | WatchMask::CREATE
            )
            .expect("failed to start watching directory");

        println!("FilesystemWatcher for {} started!", &self.dir);

        let mut buffer = [0u8; 4096];
        loop {
            let events = watcher
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if event.mask.contains(EventMask::CREATE) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory created: {:?}", event.name);
                    } else {
                        println!("File created: {:?}", event.name);
                    }
                } else if event.mask.contains(EventMask::DELETE) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory deleted: {:?}", event.name);
                    } else {
                        println!("File deleted: {:?}", event.name);
                    }
                } else if event.mask.contains(EventMask::MODIFY) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory modified: {:?}", event.name);
                    } else {
                        println!("File modified: {:?}", event.name);
                    }
                }
            }
        }
    }
}
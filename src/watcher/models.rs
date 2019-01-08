use std::fs::read_dir;
use std::path::PathBuf;
use std::collections::VecDeque;
use inotify::{Inotify, EventMask, WatchMask};
use super::error::Error;

pub enum FileType {
    Directory,
    File
}

#[derive(PartialEq)]
pub enum EventType {
    Updated,
    Deleted,
    SelfDeleted,
    Irrelevant
}

pub struct Event {
    pub path: PathBuf,
    pub file_type: FileType,
    pub event_type: EventType
}

pub struct DirectoryWatcher {
    pub path: PathBuf,
    notifier: Inotify,
    events: VecDeque<Event>
}

fn map_event(mask: EventMask) -> EventType {
    if mask.intersects(EventMask::DELETE_SELF | EventMask::MOVE_SELF) {
        return EventType::SelfDeleted;
    }

    if mask.intersects(EventMask::CREATE | EventMask::MOVED_TO | EventMask::CLOSE_WRITE) {
        return EventType::Updated;
    } else if mask.intersects(EventMask::MOVED_FROM | EventMask::DELETE) {
        return EventType::Deleted;
    }

    EventType::Irrelevant
}

impl DirectoryWatcher {
    fn new(path: PathBuf) -> Result<Self, Error> {
        let mut notifier = Inotify::init()?;

        notifier.add_watch(&path, WatchMask::ALL_EVENTS)?;

        let mut watcher = DirectoryWatcher {
            notifier,
            events: VecDeque::new(),
            path   
        };

        watcher.generate_initial_events();
        Ok(watcher)
    }

    fn generate_initial_events(&mut self) {
        if let Ok(entries) = read_dir(&self.path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        
                        let file_type = if file_type.is_dir() {
                            FileType::Directory
                        } else {
                            FileType::File
                        };

                        self.events.push_back(Event {
                            path: entry.path(),
                            event_type: EventType::Updated,
                            file_type
                        });
                    }
                }
            }
        }
    }

    fn read_events(&mut self) -> Result<usize, Error> {
        let mut buffer = [0u8; 4096];
        let events = self.notifier.read_events_blocking(&mut buffer)?;

        for event in events {
            let event_type = map_event(event.mask);

            if event_type == EventType::Irrelevant {
                continue;
            }

            let name = match event.name {
                Some(name) => name,
                None => return Err(Error::NameError)
            };

            let file_type = if event.mask.contains(EventMask::ISDIR) {
                FileType::Directory
            } else {
                FileType::File
            };

            let mut fullpath = self.path.clone();
            fullpath.push(name);

            self.events.push_back(Event {
                path: fullpath,
                event_type,
                file_type
            });
        }

        Ok(self.events.len())
    }

    pub fn get_event(&mut self) -> Result<Event, Error> {
        if self.events.len() > 0 {
            return Ok(self.events.pop_front().unwrap());
        } else {
            while self.read_events()? == 0 { }
            return Ok(self.events.pop_front().unwrap())
        }        
    }
}
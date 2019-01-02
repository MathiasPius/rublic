/*
use actix::Handler;
use crate::errors::ServiceError;
use super::FilesystemWatcher;
use super::messages::*;


impl Handler<AddWatchedDirectory> for FilesystemWatcher {
    type Result = Result<String, ServiceError>;

    fn handle(&mut self, msg: AddWatchedDirectory, _: &mut Self::Context) -> Self::Result {

        println!("Watching: {}", &msg.directory);
        self.dirs.push(msg.directory.clone());

        Ok(msg.directory)
    }
}
*/
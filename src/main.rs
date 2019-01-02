// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

extern crate crypto;
extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate rand;
extern crate inotify;
#[macro_use] extern crate diesel;
#[macro_use] extern crate failure;

mod app;
#[macro_use] mod models;
mod schema;
mod errors;
mod cryptoutil;
mod database;
mod watcher;
mod api;

use actix::prelude::*;
use actix_web::server;
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use dotenv::dotenv;
use std::env;
use crate::database::DbExecutor;
use crate::watcher::ArchiveWatcher;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let letsencrypt_archive = env::var("LETSENCRYPT_ARCHIVE").unwrap_or("/etc/letsencrypt/archive".into()).into();

    let sys = actix::System::new("Rublic");

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let database: Addr<DbExecutor>  = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let dbref = database.clone();
    let _watcher: Addr<ArchiveWatcher> = Arbiter::start(move |_| {
        ArchiveWatcher::new(dbref.clone(), letsencrypt_archive)
    });

    server::new(move || app::create_app(database.clone()))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .start();

    sys.run();
}
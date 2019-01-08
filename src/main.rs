// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

extern crate crypto;
extern crate actix;
extern crate actix_web;
extern crate actix_web_httpauth;
extern crate serde;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate rand;
extern crate inotify;
extern crate regex;
extern crate openssl;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate diesel;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

mod app;

mod config;
#[macro_use] mod models;
mod authorization;
mod schema;
mod errors;
mod cryptoutil;
mod database;
mod watcher;
mod certificates;
mod api;

use actix::prelude::*;
use actix_web::server;
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use dotenv::dotenv;
use crate::authorization::AuthorizationManager;
use crate::certificates::CertificateManager;
use crate::database::DbExecutor;
use crate::watcher::ArchiveWatcher;
use crate::config::{DATABASE_URL, LETSENCRYPT_ARCHIVE};


fn main() {
    env_logger::init();

    dotenv().ok();
    crate::config::initialize();

    let sys = actix::System::new("Rublic");

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL.to_string());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let database = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let dbref = database.clone();
    let certman = Arbiter::start(move |_| {
        CertificateManager { db: dbref.clone() }
    });

    let dbref = database.clone();
    let authman = Arbiter::start(move |_| {
        AuthorizationManager { db: dbref.clone() }
    });

    let certmanref = certman.clone();
    let dbref = database.clone();
    Arbiter::start(move |_| {
        ArchiveWatcher::new(dbref.clone(), certmanref.clone(), LETSENCRYPT_ARCHIVE.to_path_buf())
    });

    server::new(move || app::create_app(database.clone(), certman.clone(), authman.clone()))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .start();

    sys.run();
}
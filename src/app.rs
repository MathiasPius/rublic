use actix::prelude::*;
use actix_web::{middleware, App};
use crate::database::DbExecutor;
use crate::certman::CertificateManager;
//use crate::credentials::routes::RublicCredentialsRouter;
//use crate::domains::routes::RublicDomainsRouter;

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub certman: Addr<CertificateManager>
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>, certman: Addr<CertificateManager>) -> App<AppState> {
    let state = AppState { 
        db,
        certman
    };
    
    App::with_state(state)
        // setup builtin logger to get nice logging for each request
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        
        .scope("/api", |api| {
            crate::api::register(api)
        })

}
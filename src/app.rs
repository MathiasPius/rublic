use actix::prelude::*;
use actix_web::{middleware, App};
use crate::database::DbExecutor;
use crate::certman::CertificateManager;
use crate::authorization::AuthorizationManager;

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub certman: Addr<CertificateManager>,
    pub authman: Addr<AuthorizationManager>
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>, certman: Addr<CertificateManager>, authman: Addr<AuthorizationManager>) -> App<AppState> {
    let state = AppState { 
        db,
        certman,
        authman
    };
    
    App::with_state(state)
        // setup builtin logger to get nice logging for each request
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        .middleware(crate::authorization::ClaimsProviderMiddleware{ })

        .scope("/api", |api| {
            crate::api::register(api)
        })
}
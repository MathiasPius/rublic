use actix::prelude::*;
use actix_web::{middleware, App};
use crate::models::DbExecutor;
//use crate::credentials::routes::RublicCredentialsRouter;
//use crate::domains::routes::RublicDomainsRouter;

pub struct AppState {
    pub db: Addr<DbExecutor>
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    let state = AppState { 
        db
    };
    
    App::with_state(state)
        // setup builtin logger to get nice logging for each request
        .middleware(middleware::Logger::new("\"%r\" %s %b %Dms"))
        
        .scope("/api", |api| {
            crate::api::register(api)
        })

}
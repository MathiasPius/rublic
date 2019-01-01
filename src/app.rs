// app.rs
use actix::prelude::*;
use actix_web::{middleware, App, Scope};
use crate::models::DbExecutor;
use crate::credentials::routes::RublicCredentialsRouter;
use crate::domains::routes::RublicDomainsRouter;

pub struct AppState {
    pub db: Addr<DbExecutor>
}

pub trait RublicFeatureRouter {
    fn register(router: Scope<AppState>) -> Scope<AppState>;
}

trait RublicFeatureScope {
    fn register<T: RublicFeatureRouter>(self) -> Self;
}

impl RublicFeatureScope for Scope<AppState> {
    fn register<T: RublicFeatureRouter>(self) -> Self {
        T::register(self)
    }
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
            api
                .register::<RublicCredentialsRouter>()
                .register::<RublicDomainsRouter>()
        })

}
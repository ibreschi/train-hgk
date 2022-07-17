use actix_web::{App};

use crate::health::{health};

pub fn register_services(app: &App<T, B>) {
    app.service(health::health);
}

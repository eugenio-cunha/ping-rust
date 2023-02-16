use actix_web::{get, web, HttpResponse};

use crate::domain::ping_entity::PingEntity;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_ping);
}

#[get("/ping")]
async fn get_ping() -> HttpResponse {
    let ping = PingEntity::new(String::from("oppy"));

    HttpResponse::Ok().json(ping)
}

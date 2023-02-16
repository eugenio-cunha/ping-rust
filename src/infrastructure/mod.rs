use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::adapters;

pub fn server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().configure(adapters::api::ping::ping_controller::routes)).listen(listener)?.run();

    println!("🚀 Server running on http://0.0.0.0:{}", port);

    Ok(server)
}

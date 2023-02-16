use actix_web::dev::Server;
use std::net::TcpListener;

pub mod infrastructure; 
pub mod adapters;
pub mod  domain;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    infrastructure::server(listener)
}
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PingEntity {
    pub pong: String,
}

impl PingEntity {
    pub fn new(pong: String) -> Self {
        PingEntity { pong }
    }
}

pub use server::*;
mod server;

pub struct RatelimitConfig {
    pub url: String,
    pub address: SocketAddr
}
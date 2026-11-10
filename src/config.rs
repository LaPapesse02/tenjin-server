use std::env;

use tracing::{Level, event, instrument};

const DEFAULT_BIND_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT     : &str = "9797";

pub struct Config {
    pub bind_address: String,
    pub port: String
}

#[instrument]
pub fn load_config() -> Config {
    // load bind address from environment
    let address = match env::var("TENJIN_BIND_ADDR") {
        Ok(value) => value,
        Err(_) => {
            event!(Level::WARN, "No value set for bind address, defaulting to {DEFAULT_BIND_ADDR}");
            DEFAULT_BIND_ADDR.to_string()
        }
    };
    // load port from environment
    let port = match env::var("TENJIN_PORT") {
        Ok(value) => value,
        Err(_) => {
            event!(Level::WARN, "No value set for port, defaulting to {DEFAULT_PORT}");
            DEFAULT_PORT.to_string()
        }
    };

    Config {
        bind_address:address,
        port: port
    }
}
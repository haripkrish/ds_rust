use log::{info};

mod ds;
use ds::cmd_handler;
mod config;


fn main() {
    config::setup_logger();
    info!("DS & Algo - Rust");
    cmd_handler::run();
}

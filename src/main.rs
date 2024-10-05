use env_logger;
use log::{debug, error, info, warn};

mod ds;
use ds::tree::heap::{min_heap};
use min_heap::solution as min_heap_solution;

fn setup_logger() {
    env_logger::init();
}

fn main() {
    setup_logger();
    info!("DS & Algo - Rust");
    min_heap_solution::min_heap();
}

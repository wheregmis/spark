//! Native binary entry point for the counter app.

fn main() {
    env_logger::init();
    counter::run_counter();
}


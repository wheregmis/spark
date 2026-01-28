//! Native binary entry point for the hello world app.

fn main() {
    env_logger::init();
    hello_world::run_hello_world();
}

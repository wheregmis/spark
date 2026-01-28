//! Layout Gallery - Visual testing for layout features

use spark::prelude::*;

fn main() {
    env_logger::init();

    App::new()
        .with_title("Layout Gallery - Spark")
        .with_size(1000, 800)
        .with_background(Color::from_hex(0x1F2937))
        .run(build_ui);
}

fn build_ui() -> Box<dyn Widget> {
    Box::new(
        Container::new()
            .fill()
            .background(Color::from_hex(0x1F2937))
            .child(
                Text::new("Layout Gallery")
                    .size(24.0)
                    .bold()
                    .color(Color::WHITE),
            ),
    )
}

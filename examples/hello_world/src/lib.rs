//! Hello World example for Spark.

use spark::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn run_hello_world() {
    App::new()
        .with_title("Hello World")
        .with_size(640, 360)
        .with_background(Color::from_hex(0x0F172A))
        .run(|| {
            Box::new(
                Container::new()
                    .fill()
                    .center()
                    .child(
                        Text::new("Hello, WASM!")
                            .size(48.0)
                            .bold()
                            .color(Color::from_hex(0x38BDF8)),
                    ),
            )
        });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    spark::init_web();
    run_hello_world();
}

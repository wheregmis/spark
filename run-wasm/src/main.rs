fn main() {
    cargo_run_wasm::run_wasm_cli_with_css(
        r#"
        html, body {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            background: #0F172A;
        }
        canvas {
            width: 100%;
            height: 100%;
            display: block;
        }
        "#,
    );
}

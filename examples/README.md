# Examples

- `triangle`: Minimal typed pipeline demo rendering a color-animated triangle.
- `counter`: Stateful widgets with both native and WebAssembly entry points.
- `hello_world`: Minimal hello world example for native + WebAssembly.

Run (native):
```bash
cargo run -p triangle --release
cargo run -p counter --release
cargo run -p hello_world --release
```

Run (WebAssembly):
```bash
cargo run-wasm -p counter
cargo run-wasm -p hello_world
```

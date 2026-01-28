# Examples

- `triangle`: Minimal typed pipeline demo rendering a color-animated triangle.
- `demo`: Full widget showcase (layout, buttons, text input, scroll).
- `counter`: Stateful counter app (desktop + WebAssembly).
- `native-demo`: Native macOS/iOS widget integration demo.

Run:
```bash
cargo run -p demo --release
cargo run -p triangle --release
cargo run -p counter --release

# macOS/iOS only
cargo run -p native-demo --release
```

# Spark

A GPU-first cross-platform UI framework in Rust, built on `wgpu` and `winit`.

## Features

- **GPU-Accelerated Rendering** - All rendering uses wgpu with instanced drawing for shapes and text
- **Flexbox Layout** - Powered by [taffy](https://github.com/DioxusLabs/taffy) for familiar CSS-like layouts
- **Cross-Platform** - Desktop (Windows, macOS, Linux) and Web (via WebGPU)
- **Modern Text Rendering** - Using [cosmic-text](https://github.com/pop-os/cosmic-text) for font shaping
- **W3C-Compliant Events** - Using [ui-events](https://github.com/endoli/ui-events) for input handling

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│              (User code: components, state)                  │
├─────────────────────────────────────────────────────────────┤
│                    spark (facade)                           │
│              App runner, context, hooks                      │
├─────────────────────────────────────────────────────────────┤
│  spark-widgets  │  spark-input   │  spark-layout          │
│  (Button, Input) │  (Events, Focus)│  (Flexbox/taffy)        │
├─────────────────────────────────────────────────────────────┤
│                    spark-render                             │
│     DrawList, ShapePass, TextPass, batching, sorting         │
├─────────────────────────────────────────────────────────────┤
│                    spark-core                               │
│     Pipeline<U>, wgpu init, uniform buffers, shaders         │
├─────────────────────────────────────────────────────────────┤
│                    Platform (wgpu + winit)                   │
│            Desktop (Vulkan/Metal/DX12) | Web (WebGPU)        │
└─────────────────────────────────────────────────────────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `spark` | Main facade crate with App runner |
| `spark-core` | GPU primitives, pipelines, vertex buffers |
| `spark-render` | DrawList, shape/text rendering passes |
| `spark-layout` | Flexbox layout via taffy |
| `spark-text` | Font loading, text shaping, glyph atlas |
| `spark-input` | Event types, focus management, hit testing |
| `spark-widgets` | Widget trait and basic widgets |

## Quick Start

```rust
use spark::prelude::*;

fn main() {
    App::new()
        .with_title("My App")
        .with_size(800, 600)
        .run(|| {
            Box::new(
                Container::new()
                    .fill()
                    .center()
                    .gap(16.0)
                    .child(Button::new("Click me!"))
                    .child(TextInput::new().placeholder("Enter text..."))
            )
        });
}
```

## Widgets

- **Container** - Flexbox container for layout
- **Button** - Clickable button with hover/press states
- **TextInput** - Single-line text input with cursor
- **Scroll** - Scrollable container

## Try It

```bash
# Run the demo
cargo run -p demo --release

# Run the original triangle example
cargo run -p triangle --release
```

## Frame Loop

```
1. EVENT PHASE
   - Collect winit events → InputEvent
   - Dispatch to focused widget / hit-test target
   - Update widget state if needed

2. LAYOUT PHASE
   - Traverse widget tree, collect taffy::Style
   - Call taffy.compute_layout()
   - Produce ComputedLayout tree

3. PAINT PHASE
   - Traverse widget tree with ComputedLayout
   - Each widget emits DrawCommands to DrawList

4. RENDER PHASE
   - Sort DrawList by pipeline (shapes, then text)
   - Batch vertices into GPU buffers
   - Encode render passes
   - Submit to queue, present frame
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `wgpu` | GPU abstraction |
| `winit` | Windowing (desktop + web) |
| `taffy` | Flexbox layout engine |
| `cosmic-text` | Text shaping + rasterization |
| `ui-events` | W3C-compliant input events |
| `bytemuck` | Safe GPU buffer casts |
| `glam` | Math types (Vec2, Mat4) |

## References

- [Makepad](https://github.com/makepad/makepad) - GPU-first UI in Rust
- [gpui](https://github.com/zed-industries/zed) - Zed's UI framework
- [wgpu](https://github.com/gfx-rs/wgpu) - Cross-platform GPU abstraction

## License

MIT

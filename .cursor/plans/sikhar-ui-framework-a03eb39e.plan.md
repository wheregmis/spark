<!-- a03eb39e-7db8-441a-aa09-0023fcb67545 b3be7cee-54fd-4757-99a2-410b75a657fe -->
# Sikhar UI Framework Architecture

## Crate Structure

```
sikhar/
├── crates/
│   ├── sikhar-core/        # GPU primitives, pipelines, draw commands (existing)
│   ├── sikhar-render/      # Draw list, batching, shape/text passes
│   ├── sikhar-layout/      # Flexbox via taffy, computed layout
│   ├── sikhar-text/        # Font atlas, text shaping (cosmic-text or fontdue)
│   ├── sikhar-input/       # Events, focus, hit-testing
│   ├── sikhar-widgets/     # Button, TextInput, Scroll, Container
│   └── sikhar/             # Facade crate, re-exports, App runner
```

## Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│              (User code: components, state)                  │
├─────────────────────────────────────────────────────────────┤
│                    sikhar (facade)                           │
│              App runner, context, hooks                      │
├─────────────────────────────────────────────────────────────┤
│  sikhar-widgets  │  sikhar-input   │  sikhar-layout          │
│  (Button, Input) │  (Events, Focus)│  (Flexbox/taffy)        │
├─────────────────────────────────────────────────────────────┤
│                    sikhar-render                             │
│     DrawList, ShapePass, TextPass, batching, sorting         │
├─────────────────────────────────────────────────────────────┤
│                    sikhar-core                               │
│     Pipeline<U>, wgpu init, uniform buffers, shaders         │
├─────────────────────────────────────────────────────────────┤
│                    Platform (wgpu + winit)                   │
│            Desktop (Vulkan/Metal/DX12) | Web (WebGPU)        │
└─────────────────────────────────────────────────────────────┘
```

## Key Components

### 1. sikhar-core (enhance existing)

Extend current pipeline to support:

- **Vertex buffers** for instanced quads/shapes
- **Multiple pipeline types**: `ShapePipeline`, `TextPipeline`
- **Global uniforms**: viewport size, time, scale factor

Key types:

```rust
// Vertex for shape instances
#[repr(C)]
struct ShapeVertex {
    pos: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    corner_radius: f32,
    // ...
}

// Pipeline variants
enum RenderPipeline {
    Shape(Pipeline<ShapeUniforms>),
    Text(Pipeline<TextUniforms>),
}
```

### 2. sikhar-render (new)

**DrawList**: Collect draw commands from the component tree.

```rust
pub enum DrawCommand {
    Rect { bounds: Rect, color: Color, radius: f32 },
    Text { bounds: Rect, glyphs: Vec<GlyphInstance> },
    Clip { bounds: Rect },
    PopClip,
}

pub struct DrawList {
    commands: Vec<DrawCommand>,
}
```

**Renderer**: Sort by pipeline, batch, encode to wgpu.

```rust
impl Renderer {
    pub fn render(&mut self, draw_list: &DrawList, encoder: &mut CommandEncoder);
}
```

### 3. sikhar-layout (new)

Wrap [taffy](https://github.com/DioxusLabs/taffy) for flexbox:

```rust
pub struct LayoutTree {
    taffy: taffy::TaffyTree,
    nodes: HashMap<WidgetId, taffy::NodeId>,
}

pub struct ComputedLayout {
    pub bounds: Rect,
    pub children: Vec<ComputedLayout>,
}
```

### 4. sikhar-text (new)

Font loading + atlas + shaping:

- Use `cosmic-text` or `fontdue` for shaping
- GPU texture atlas for glyph cache
- Text layout integration with flexbox (measure text for layout)
```rust
pub struct TextSystem {
    atlas: GlyphAtlas,
    font_system: FontSystem,
}

impl TextSystem {
    pub fn shape(&mut self, text: &str, style: &TextStyle) -> ShapedText;
    pub fn rasterize(&mut self, queue: &Queue) -> &TextureView;
}
```


### 5. sikhar-input (new)

Event handling + hit testing:

```rust
pub enum InputEvent {
    MouseMove { pos: Point },
    MouseDown { button: MouseButton, pos: Point },
    MouseUp { button: MouseButton, pos: Point },
    KeyDown { key: Key, modifiers: Modifiers },
    KeyUp { key: Key },
    TextInput { text: String },
}

pub struct FocusManager {
    focused: Option<WidgetId>,
}

pub fn hit_test(layout: &ComputedLayout, pos: Point) -> Option<WidgetId>;
```

### 6. sikhar-widgets (new)

Retained component tree with props/state:

```rust
pub trait Widget {
    fn layout(&self, ctx: &mut LayoutContext) -> taffy::Style;
    fn paint(&self, ctx: &mut PaintContext, layout: &ComputedLayout);
    fn event(&mut self, ctx: &mut EventContext, event: &InputEvent) -> EventResponse;
}

// Basic widgets
pub struct Button { label: String, on_click: Callback }
pub struct TextInput { value: String, on_change: Callback }
pub struct Scroll { offset: f32, content: Box<dyn Widget> }
pub struct Container { children: Vec<Box<dyn Widget>>, style: Style }
```

### 7. sikhar (facade + app runner)

Main entry point and application lifecycle:

```rust
pub struct App<S> {
    state: S,
    root: Box<dyn Widget>,
}

impl<S> App<S> {
    pub fn run(self) -> !;
}

// Context passed to widgets
pub struct Context<'a> {
    pub layout: &'a LayoutTree,
    pub text: &'a mut TextSystem,
    pub draw_list: &'a mut DrawList,
}
```

## Frame Loop (Layout → Paint → Encode → Submit)

```
┌──────────────────────────────────────────────────────────┐
│ 1. EVENT PHASE                                            │
│    - Collect winit events → InputEvent                    │
│    - Dispatch to focused widget / hit-test target         │
│    - Update widget state if needed                        │
├──────────────────────────────────────────────────────────┤
│ 2. LAYOUT PHASE                                           │
│    - Traverse widget tree, collect taffy::Style           │
│    - Call taffy.compute_layout()                          │
│    - Produce ComputedLayout tree                          │
├──────────────────────────────────────────────────────────┤
│ 3. PAINT PHASE                                            │
│    - Traverse widget tree with ComputedLayout             │
│    - Each widget emits DrawCommands to DrawList           │
├──────────────────────────────────────────────────────────┤
│ 4. RENDER PHASE                                           │
│    - Sort DrawList by pipeline (shapes, then text)        │
│    - Batch vertices into GPU buffers                      │
│    - Encode render passes                                 │
│    - Submit to queue, present frame                       │
└──────────────────────────────────────────────────────────┘
```

## Web Platform Support

- **Conditional compilation**: `#[cfg(target_arch = "wasm32")]`
- **winit** already supports web via `web-sys`
- **wgpu** compiles to WebGPU on wasm32
- **Async init**: Use `wasm-bindgen-futures` for adapter/device requests
- **Canvas sizing**: Handle `<canvas>` resize events

Add to workspace:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlCanvasElement"] }
console_error_panic_hook = "0.1"
```

## Key Dependencies

| Crate | Purpose |

|-------|---------|

| `wgpu` | GPU abstraction |

| `winit` | Windowing (desktop + web) |

| `taffy` | Flexbox layout engine |

| `cosmic-text` or `fontdue` | Text shaping + rasterization |

| `bytemuck` | Safe GPU buffer casts |

| `glam` | Math types (Vec2, Mat4) |

## Implementation Order

Phase 1 focuses on the rendering foundation, Phase 2 on layout and text, Phase 3 on the widget system, and Phase 4 on web support.

### To-dos

- [ ] Add vertex buffer support + instanced quad rendering to sikhar-core
- [ ] Create sikhar-render with DrawList, DrawCommand, shape batching
- [ ] Implement ShapePipeline with rounded rect shader
- [ ] Create sikhar-layout wrapping taffy for flexbox
- [ ] Create sikhar-text with font atlas and glyph shaping
- [ ] Implement TextPipeline with atlas sampling shader
- [ ] Create sikhar-input with event types and hit-testing
- [ ] Define Widget trait and component tree in sikhar-widgets
- [ ] Implement Button, TextInput, Scroll, Container widgets
- [ ] Create sikhar facade with App runner and frame loop
- [ ] Add WASM/WebGPU support with conditional compilation
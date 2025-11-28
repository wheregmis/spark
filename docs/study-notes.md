# Study Notes: GPU UI + wgpu Ecosystem

Goal: Extract patterns (not copy) from Makepad, gpui, wgpu, naga.

## Focus Areas
- Render graph: stages, passes, dependencies, data flow.
- UI primitives: how buffers (vertex/index/uniform/storage) map to widgets.
- Layout → draw commands → pipelines: transforming computed layout into batched GPU work.
- Unsafe boundaries: where and why, and how they’re isolated.

## Makepad (Rust)
- GPU-first immediate-ish UI; custom shading language compiled to GPU backends.
- Aggressive batching: atlas textures, geometry buffers; shader-driven styling.
- Tight coupling of UI primitives to shader params; passes for text, shapes, effects.
- Unsafe primarily in FFI/graphics interop; otherwise types guide buffer usage.

## gpui (Zed’s UI framework)
- ECS-ish views: structured composition + async updates.
- Clear separation: layout computation → render lists → GPU submission.
- Strong focus on determinism and incremental updates.
- Typed resources for pipelines, textures, buffers; robust validation around state.

## wgpu
- Safe abstraction over Vulkan/Metal/DX12/OpenGL/WebGPU.
- Pipeline-centric design: bind group layouts, render pipelines, command encoders.
- WGSL first-class; validation via naga.
- Device/Queue, swapchain rendering, command buffers, passes.

## naga
- IR + validators; supports WGSL/GLSL/SPIR-V; cross-backend translation.
- Use for extra validation and portability if needed beyond wgpu.

## Patterns to Emulate
- Typed pipelines: Rust types wrap shader entry points + bind groups.
- Uniform API: strongly typed uniform structs → `bytemuck` → GPU buffers.
- Batched draw lists: collect draw commands from layout stage, group by pipeline.
- Minimal unsafe: only at FFI boundaries; use newtypes to isolate raw handles.
- Clear stages: Layout → Build DrawList → Encode → Submit.

## Next Steps
- WGSL-first shader story with typed wrappers.
- Minimal demo: triangle/quad with a typed uniform struct (color/transform).
- Define `Pipeline<TUniform>` generic type with layout and bind creation.

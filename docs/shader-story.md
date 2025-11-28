# Shader Story

Decision: Start WGSL-first, build typed Rust wrappers around pipelines and uniforms. Explore Rust-DSL → WGSL later via proc-macros.

## Why WGSL-first
- Cross-platform: native + web via WebGPU.
- Strong validation through wgpu + naga.
- Faster time-to-first-demo; less compiler work.

## Typed Pipeline Concept
- `Pipeline<U>` owns: `wgpu::RenderPipeline`, `wgpu::BindGroupLayout`, `wgpu::BindGroup`, and a GPU uniform buffer for `U`.
- `U` implements `bytemuck::{Pod, Zeroable}`; updated via queue writes.
- Shader entry points defined in WGSL; wrapper encodes layout compatibility.

## Uniform API
- Define uniform structs in Rust; mirror WGSL layout.
- Use `#[repr(C)]` and padding rules to match WGSL alignment.
- Provide helpers: `Pipeline::update_uniforms(&Queue, &U)`.

## Future: Rust-DSL
- Proc-macro to restrict subset and emit WGSL/SPIR-V.
- Benefits: single-source-of-truth types; editor tooling.
- Cost: significant, defer until core framework stable.

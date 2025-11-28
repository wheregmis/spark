//! Sikhar Core - GPU primitives, pipelines, and low-level rendering.

pub mod buffer;
pub mod pipeline;
pub mod types;
pub mod vertex;
pub mod wgpu_init;

// Re-exports
pub use buffer::{DynamicBuffer, QuadBuffers, StaticBuffer};
pub use pipeline::{Pipeline, UniformBuffer};
pub use types::{Color, GlobalUniforms, Point, Rect};
pub use vertex::{GlyphInstance, ShapeInstance, Vertex2D};
pub use wgpu_init::{init_wgpu, SurfaceState};

// Re-export wgpu and glam for convenience
pub use glam;
pub use wgpu;

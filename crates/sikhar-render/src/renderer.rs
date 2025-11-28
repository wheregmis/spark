//! Main renderer that processes draw lists and issues GPU commands.

use crate::{DrawCommand, DrawList, ShapePass, TextPass};
use sikhar_core::{GlobalUniforms, Rect};
use sikhar_text::GlyphAtlas;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};

/// The main renderer that processes draw lists and renders to the screen.
pub struct Renderer {
    shape_pass: ShapePass,
    text_pass: TextPass,
    globals: GlobalUniforms,
    clip_stack: Vec<Rect>,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self {
            shape_pass: ShapePass::new(device, format),
            text_pass: TextPass::new(device, format),
            globals: GlobalUniforms::default(),
            clip_stack: Vec::new(),
        }
    }

    /// Update global uniforms (call once per frame before rendering).
    pub fn set_viewport(&mut self, width: f32, height: f32, scale_factor: f32) {
        self.globals.viewport_size = [width, height];
        self.globals.scale_factor = scale_factor;
    }

    /// Update time uniform.
    pub fn set_time(&mut self, time: f32) {
        self.globals.time = time;
    }

    /// Process a draw list and prepare GPU resources.
    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        draw_list: &DrawList,
        atlas: &GlyphAtlas,
    ) {
        self.shape_pass.clear();
        self.text_pass.clear();
        self.clip_stack.clear();

        for command in draw_list.commands() {
            match command {
                DrawCommand::Rect {
                    bounds,
                    color,
                    corner_radius,
                    border_width,
                    border_color,
                } => {
                    // Apply clipping if needed
                    let clipped_bounds = if let Some(clip) = self.clip_stack.last() {
                        match bounds.intersection(clip) {
                            Some(b) => b,
                            None => continue, // Fully clipped, skip
                        }
                    } else {
                        *bounds
                    };

                    self.shape_pass.add_rect(
                        clipped_bounds,
                        color.to_array(),
                        *corner_radius,
                        *border_width,
                        border_color.to_array(),
                    );
                }
                DrawCommand::Text { glyphs } => {
                    // TODO: Apply clipping to glyphs
                    self.text_pass.add_glyphs(glyphs);
                }
                DrawCommand::PushClip { bounds } => {
                    // Intersect with current clip if any
                    let new_clip = if let Some(current) = self.clip_stack.last() {
                        bounds.intersection(current).unwrap_or(Rect::ZERO)
                    } else {
                        *bounds
                    };
                    self.clip_stack.push(new_clip);
                }
                DrawCommand::PopClip => {
                    self.clip_stack.pop();
                }
            }
        }

        // Update GPU buffers
        self.shape_pass.prepare(device, queue, &self.globals);
        self.text_pass.prepare(device, queue, &self.globals, atlas);
    }

    /// Render to the given texture view.
    pub fn render(
        &self,
        encoder: &mut CommandEncoder,
        target: &TextureView,
        clear_color: wgpu::Color,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sikhar_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Render shapes first (background)
        self.shape_pass.render(&mut render_pass);

        // Render text on top
        self.text_pass.render(&mut render_pass);
    }

    /// Get the number of shape instances being rendered.
    pub fn shape_count(&self) -> usize {
        self.shape_pass.instance_count()
    }

    /// Get the number of glyph instances being rendered.
    pub fn glyph_count(&self) -> usize {
        self.text_pass.instance_count()
    }
}


//! Text shaping and layout system using cosmic-text.

use crate::atlas::{CachedGlyph, GlyphAtlas, GlyphKey};
use cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache,
};
use sikhar_core::{Color, GlyphInstance};
use wgpu::{Device, Queue};

/// Text style configuration.
#[derive(Clone, Debug)]
pub struct TextStyle {
    /// Font family name.
    pub family: String,
    /// Font size in pixels.
    pub font_size: f32,
    /// Line height multiplier.
    pub line_height: f32,
    /// Text color.
    pub color: Color,
    /// Whether the text is bold.
    pub bold: bool,
    /// Whether the text is italic.
    pub italic: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            family: String::from("sans-serif"),
            font_size: 16.0,
            line_height: 1.2,
            color: Color::BLACK,
            bold: false,
            italic: false,
        }
    }
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_family(mut self, family: impl Into<String>) -> Self {
        self.family = family.into();
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
}

/// Result of text shaping - positioned glyphs ready for rendering.
#[derive(Clone, Debug, Default)]
pub struct ShapedText {
    /// Glyph instances ready for GPU rendering.
    pub glyphs: Vec<GlyphInstance>,
    /// Total width of the shaped text.
    pub width: f32,
    /// Total height of the shaped text.
    pub height: f32,
}

impl ShapedText {
    /// Check if the shaped text has any glyphs.
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }
}

/// The text system manages fonts, shaping, and glyph caching.
pub struct TextSystem {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: GlyphAtlas,
}

impl TextSystem {
    /// Create a new text system.
    pub fn new(device: &Device) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let atlas = GlyphAtlas::new(device, 1024, 1024);

        Self {
            font_system,
            swash_cache,
            atlas,
        }
    }

    /// Get a reference to the font system for loading fonts.
    pub fn font_system(&self) -> &FontSystem {
        &self.font_system
    }

    /// Get a mutable reference to the font system for loading fonts.
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }

    /// Get the glyph atlas.
    pub fn atlas(&self) -> &GlyphAtlas {
        &self.atlas
    }

    /// Shape and position text for rendering.
    pub fn shape(
        &mut self,
        device: &Device,
        queue: &Queue,
        text: &str,
        style: &TextStyle,
        max_width: Option<f32>,
    ) -> ShapedText {
        if text.is_empty() {
            return ShapedText::default();
        }

        let metrics = Metrics::new(style.font_size, style.font_size * style.line_height);

        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        // Set buffer width for wrapping
        let width = max_width.unwrap_or(f32::MAX);
        buffer.set_size(&mut self.font_system, Some(width), None);

        // Build attributes
        let weight = if style.bold {
            cosmic_text::Weight::BOLD
        } else {
            cosmic_text::Weight::NORMAL
        };

        let style_attr = if style.italic {
            cosmic_text::Style::Italic
        } else {
            cosmic_text::Style::Normal
        };

        let attrs = Attrs::new()
            .family(Family::Name(&style.family))
            .weight(weight)
            .style(style_attr);

        buffer.set_text(&mut self.font_system, text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);

        // Collect glyph instances
        let mut glyphs = Vec::new();
        let mut total_width: f32 = 0.0;
        let mut total_height: f32 = 0.0;
        let color_arr = style.color.to_array();

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, 0.0), 1.0);

                // Get or create cached glyph
                let key = GlyphKey::new(
                    glyph.font_id,
                    glyph.glyph_id,
                    style.font_size,
                );

                let cached = if let Some(cached) = self.atlas.get(&key) {
                    *cached
                } else {
                    // Rasterize the glyph
                    if let Some(image) = self.swash_cache.get_image(
                        &mut self.font_system,
                        physical_glyph.cache_key,
                    ) {
                        let cached = self.atlas.insert(
                            queue,
                            key,
                            image.placement.width,
                            image.placement.height,
                            image.placement.left,
                            image.placement.top,
                            &image.data,
                        );

                        match cached {
                            Some(c) => c,
                            None => {
                                // Atlas full, clear and retry
                                self.atlas.clear();
                                self.atlas = GlyphAtlas::new(device, 2048, 2048);
                                continue;
                            }
                        }
                    } else {
                        // Create empty glyph for spaces
                        CachedGlyph {
                            uv_x: 0.0,
                            uv_y: 0.0,
                            uv_width: 0.0,
                            uv_height: 0.0,
                            width: 0,
                            height: 0,
                            offset_x: 0,
                            offset_y: 0,
                        }
                    }
                };

                // Skip empty glyphs
                if cached.width == 0 || cached.height == 0 {
                    continue;
                }

                let x = physical_glyph.x as f32 + cached.offset_x as f32;
                let y = run.line_y + physical_glyph.y as f32 - cached.offset_y as f32;

                glyphs.push(GlyphInstance {
                    pos: [x, y],
                    size: [cached.width as f32, cached.height as f32],
                    uv_pos: [cached.uv_x, cached.uv_y],
                    uv_size: [cached.uv_width, cached.uv_height],
                    color: color_arr,
                });

                total_width = total_width.max(x + cached.width as f32);
            }

            total_height = total_height.max(run.line_y + run.line_height);
        }

        ShapedText {
            glyphs,
            width: total_width,
            height: total_height,
        }
    }

    /// Measure text without rasterizing (faster for layout).
    pub fn measure(&mut self, text: &str, style: &TextStyle, max_width: Option<f32>) -> (f32, f32) {
        if text.is_empty() {
            return (0.0, style.font_size * style.line_height);
        }

        let metrics = Metrics::new(style.font_size, style.font_size * style.line_height);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        let width = max_width.unwrap_or(f32::MAX);
        buffer.set_size(&mut self.font_system, Some(width), None);

        let weight = if style.bold {
            cosmic_text::Weight::BOLD
        } else {
            cosmic_text::Weight::NORMAL
        };

        let style_attr = if style.italic {
            cosmic_text::Style::Italic
        } else {
            cosmic_text::Style::Normal
        };

        let attrs = Attrs::new()
            .family(Family::Name(&style.family))
            .weight(weight)
            .style(style_attr);

        buffer.set_text(&mut self.font_system, text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut total_width: f32 = 0.0;
        let mut total_height: f32 = 0.0;

        for run in buffer.layout_runs() {
            total_width = total_width.max(run.line_w);
            total_height = total_height.max(run.line_y + run.line_height);
        }

        (total_width, total_height)
    }
}


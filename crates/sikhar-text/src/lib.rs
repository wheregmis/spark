//! Sikhar Text - Font loading, text shaping, and glyph atlas.

mod atlas;
mod system;

pub use atlas::GlyphAtlas;
pub use system::{ShapedText, TextStyle, TextSystem};

// Re-export cosmic-text for font configuration
pub use cosmic_text;


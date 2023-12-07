use crate::{font_shared, math::IRect};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AtlasMeta {
    pub atlas_name: String,
    pub texture_file: String,
    pub width: u32,
    pub height: u32,

    // Sprites
    pub sprites: Vec<IRect>,

    // Fonts
    pub fonts: Vec<FontMeta>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FontMeta {
    pub name: String,
    pub animation: font_shared::TextCharacterAnimation,
    pub num_animation_frames: u32,
    pub line_height: u32,
    pub base_line_y: u32,
    pub chars: Vec<CharMeta>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharMeta {

    /// The first sprite index (in the sprites vector of the altas).
    /// The following `num_animation_frames - 1` sprites are the other
    /// animation frames of this character.
    pub first_sprite_index: u32,
    pub char_code: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub x_advance: u32,
}

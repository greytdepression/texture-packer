use std::collections::HashMap;

use anyhow::Context;

use crate::{
    font_shared,
    intermediates::{font::FontIntermediate, texture_atlas::TextureAtlas},
    math::IRect,
};

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
    pub x_offset: i32,
    pub y_offset: i32,
    pub x_advance: i32,
}

impl AtlasMeta {
    pub fn from_texture_atlas(
        name: String,
        texture_file: String,
        atlas: &TextureAtlas,
    ) -> anyhow::Result<Self> {
        let mut builder = Self {
            atlas_name: name,
            texture_file,
            width: atlas.final_image_bounds.width as u32,
            height: atlas.final_image_bounds.height as u32,
            sprites: vec![],
            fonts: vec![],
        };

        let mut bounds_map: HashMap<(usize, usize), IRect> =
            HashMap::with_capacity(atlas.sprite_bounds.len());

        for &(asset_id, sprite_id, bounds) in atlas.sprite_bounds.iter() {
            bounds_map.insert((asset_id, sprite_id), bounds);
        }

        // Insert fonts
        for (index, font) in atlas.fonts.iter().enumerate() {
            let asset_id = atlas.get_font_asset_id(index);
            builder
                .insert_font(font, asset_id, &bounds_map)
                .with_context(|| format!("Failed to insert font #{index} '{}'", &font.name))?;
        }

        Ok(builder)
    }

    fn insert_font(
        &mut self,
        font: &FontIntermediate,
        asset_id: usize,
        bounds_map: &HashMap<(usize, usize), IRect>,
    ) -> anyhow::Result<()> {
        let mut font_meta = FontMeta {
            name: font.name.clone(),
            animation: font.animation,
            num_animation_frames: font.num_frames,
            line_height: font.line_height as u32,
            base_line_y: font.base as u32,
            chars: vec![],
        };

        #[derive(Debug, Clone)]
        struct CharMetaBuilder {
            x_offset: i32,
            y_offset: i32,
            x_advance: i32,
            frame_indices: Vec<(usize, u32)>,
        }

        // We need to combine the info from char sprites of the same character
        let mut char_builders: HashMap<u32, CharMetaBuilder> = HashMap::new();

        for (index, char_sprite) in font.chars.iter().enumerate() {
            if let Some(builder) = char_builders.get_mut(&char_sprite.char_code) {
                assert_eq!(builder.x_offset, char_sprite.x_offset);
                assert_eq!(builder.y_offset, char_sprite.y_offset);
                assert_eq!(builder.x_advance, char_sprite.x_advance);

                builder.frame_indices.push((index, char_sprite.frame));
            } else {
                char_builders.insert(
                    char_sprite.char_code,
                    CharMetaBuilder {
                        x_offset: char_sprite.x_offset,
                        y_offset: char_sprite.y_offset,
                        x_advance: char_sprite.x_advance,
                        frame_indices: vec![(index, char_sprite.frame)],
                    },
                );
            }
        }

        let mut char_builders: Vec<_> = char_builders
            .iter()
            .map(|(code, builder)| (*code, builder.clone()))
            .collect();

        char_builders.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

        // Build the CharMetas from the CharMetaBuilders
        for (char_code, builder) in char_builders.iter_mut() {
            // Sort the frame indices
            builder.frame_indices.sort_by(|a, b| a.1.cmp(&b.1));

            assert_eq!(builder.frame_indices.len(), font.num_frames as usize);

            let first_sprite_index = self.sprites.len() as u32;

            font_meta.chars.push(CharMeta {
                first_sprite_index,
                char_code: *char_code,
                x_offset: builder.x_offset,
                y_offset: builder.y_offset,
                x_advance: builder.x_advance,
            });

            // Now actually push these sprites
            for (i, &(sprite_index, frame_index)) in builder.frame_indices.iter().enumerate() {
                assert_eq!(i as u32, frame_index);

                let &bounds = bounds_map.get(&(asset_id, sprite_index)).unwrap();

                self.sprites.push(bounds);
            }
        }

        self.fonts.push(font_meta);

        Ok(())
    }
}

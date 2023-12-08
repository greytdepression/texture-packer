use anyhow::Context;
use image::{Rgba, SubImage};

use crate::{
    error::Ewwow,
    font_shared,
    math::ISize,
    sources::{SourceId, SourceSprite, Sources},
};

use super::texture_atlas::Atlasable;

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterSprite {
    pub char_code: u32,
    pub sprite: SourceSprite,
    pub frame: u32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub x_advance: i32,
}

#[derive(Debug, Clone)]
pub struct FontIntermediate {
    pub name: String,
    pub animation: font_shared::TextCharacterAnimation,
    pub num_frames: u32,
    pub line_height: i32,
    pub base: i32,
    pub chars: Vec<CharacterSprite>,
}

impl FontIntermediate {
    pub fn from_fnt(fnt_src_id: SourceId, srcs: &Sources) -> anyhow::Result<Self> {
        let fnt = srcs
            .get_fnt(fnt_src_id)
            .with_context(|| format!("Failed to load fnt file {fnt_src_id:?}"))?;

        let mut chars: Vec<CharacterSprite> = Vec::with_capacity(fnt.chars.len());

        for c in fnt.chars.iter() {
            chars.push(CharacterSprite {
                char_code: c.id,
                sprite: fnt.get_character_sprite(c.id, srcs).with_context(|| {
                    format!(
                        "Failed to generate source sprite of character '{}' (#{}) for font {}",
                        char_code_as_printable(c.id),
                        c.id,
                        &fnt.info.face,
                    )
                })?,
                frame: 0,
                x_offset: c.x_offset,
                y_offset: c.y_offset,
                x_advance: c.x_advance,
            });
        }

        Ok(Self {
            name: fnt.info.face.clone(),
            animation: font_shared::TextCharacterAnimation::NoAnimation,
            num_frames: 1,
            line_height: fnt.common.line_height,
            base: fnt.common.base,
            chars,
        })
    }

    pub fn render_text(&self, text: &str, srcs: &Sources) -> anyhow::Result<image::RgbaImage> {
        let mut curr_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut max_x = 0;

        // Determine the bounds
        for ch in text.chars() {
            let char_code = ch as u32;

            let char_info = self.chars
                .iter()
                .find(|&cs| cs.char_code == char_code)
                .ok_or(Ewwow)
                .with_context(|| format!(
                    "Failed to render '{text}' as font '{}' does not have a sprite for '{ch}' (char code #{char_code})",
                    &self.name
                ))?;

            let curr_min_y = char_info.y_offset;
            let curr_max_y = char_info.y_offset + char_info.sprite.height;
            let curr_max_x = curr_x + char_info.x_offset + char_info.sprite.width;

            min_y = min_y.min(curr_min_y);
            max_y = max_y.max(curr_max_y);
            max_x = max_x.max(curr_max_x);

            curr_x += char_info.x_advance;
        }

        // Make the image buffer
        let mut buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::RgbaImage::new(max_x as u32, self.line_height as u32);

        // Draw base line
        let base_line_color = Rgba::<u8>([128, 128, 128, 255]);

        for x in 0..max_x as u32 {
            if x % 3 != 2 {
                buffer.put_pixel(x, self.base as u32, base_line_color);
            }
        }

        // Paste characters
        curr_x = 0;
        for ch in text.chars() {
            let char_code = ch as u32;

            let char_info = self
                .chars
                .iter()
                .find(|&cs| cs.char_code == char_code)
                .unwrap();

            let x = (curr_x + char_info.x_offset) as i64;
            let y = char_info.y_offset as i64;

            let character_img = char_info.sprite
                .get_image(srcs)
                .with_context(|| format!(
                    "Failed to retrieve character sprite image for '{ch}' (code ${char_code}) for font '{}'",
                    &self.name
                ))?;

            image::imageops::overlay(&mut buffer, &character_img.to_image(), x, y);

            curr_x += char_info.x_advance;
        }

        Ok(buffer)
    }
}

fn char_code_as_printable(code: u32) -> char {
    let c = char::from_u32(code).unwrap_or(0 as char);

    if c.is_control() {
        '⌧'
    } else {
        c
    }
}

impl Atlasable for FontIntermediate {
    fn get_sprite_sizes(&self) -> Vec<ISize> {
        self.chars
            .iter()
            .map(|ch| ISize::new(ch.sprite.width, ch.sprite.height))
            .collect()
    }

    fn get_sprite_texture(&self, index: usize, srcs: &Sources) -> anyhow::Result<image::RgbaImage> {
        Ok(self.chars[index]
            .get_sprite_texture_view(srcs)
            .with_context(|| {
                format!(
                    "Failed to get the texture view of a character sprite #{} of font '{}'",
                    self.chars[index].char_code, &self.name
                )
            })?
            .to_image())
    }
}

impl CharacterSprite {
    pub fn get_sprite_texture_view<'s>(
        &self,
        srcs: &'s Sources,
    ) -> anyhow::Result<SubImage<&'s image::RgbaImage>> {
        self.sprite.get_image(srcs)
    }
}

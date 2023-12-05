use anyhow::Context;
use image::SubImage;

use crate::{
    sources::{SourceId, SourceSprite, Sources},
    texture_atlas::{Atlasable, ISize},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCharacterAnimation {
    NoAnimation,
}

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
pub struct Font {
    pub name: String,
    pub animation: TextCharacterAnimation,
    pub num_frames: u32,
    pub line_height: i32,
    pub base: i32,
    pub chars: Vec<CharacterSprite>,
}

impl Font {
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
            animation: TextCharacterAnimation::NoAnimation,
            num_frames: 0,
            line_height: fnt.common.line_height,
            base: fnt.common.base,
            chars,
        })
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

impl Atlasable for Font {
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

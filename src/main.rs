#![allow(dead_code)]
#![feature(error_generic_member_access)]

use anyhow::Context;
use intermediates::{font, texture_atlas::TextureAtlas};

mod error;
mod inputs;
mod intermediates;
mod math;
mod outputs;
mod sources;
mod font_shared;

fn main() -> anyhow::Result<()> {
    let mut sources = sources::Sources::new();

    let m5x7_id = sources
        .try_load_source("assets/m5x7.fnt")
        .with_context(|| format!("Failed to load 'm5x7.fnt'"))?;
    let font = font::FontIntermediate::from_fnt(m5x7_id, &sources)?;

    let test_text = font.render_text("Hewwo uwq, gg", &sources)?;
    test_text
        .save("test-text.png")
        .context("Failed to save test text rendering")?;

    let m5x7_color_id = sources
        .try_load_source("assets/m5x7-color.fnt")
        .with_context(|| format!("Failed to load 'm5x7-color.fnt'"))?;
    let font_color = font::FontIntermediate::from_fnt(m5x7_color_id, &sources)?;

    let mut atlas = TextureAtlas::new(math::IMargins::uniform(0));
    atlas.with_font(font);
    atlas.with_font(font_color);

    atlas.load_sizes();
    atlas.pack();

    let atlas_image = atlas
        .build_image(&sources)
        .context("Failed to build atlas image")?;

    atlas_image
        .save("atlas.png")
        .context("Failed to save atlas image")?;

    Ok(())
}

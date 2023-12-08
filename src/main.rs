#![allow(dead_code)]
#![feature(error_generic_member_access)]

use anyhow::Context;
use intermediates::{font, texture_atlas::TextureAtlas};
use outputs::atlas_meta::AtlasMeta;

mod error;
mod font_shared;
mod inputs;
mod intermediates;
mod math;
mod outputs;
mod sources;

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

    let atlas_meta = AtlasMeta::from_texture_atlas(
        "font-atlas".to_string(),
        "atlas.png".to_string(),
        &atlas
    ).context("Failed to generate AtlasMeta from texture atlas")?;

    let atlas_meta_json = serde_json::to_string_pretty(&atlas_meta)
        .context("Failed to JSON serialize atlas meta")?;

    std::fs::write("font.json", atlas_meta_json)
        .context("Failed to write JSON file")?;

    let atlas_meta_rmp = rmp_serde::to_vec(&atlas_meta).unwrap();

    std::fs::write("atlas.rmp", atlas_meta_rmp)
        .context("Failed to write RMP file")?;

    Ok(())
}

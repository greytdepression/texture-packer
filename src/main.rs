#![feature(error_generic_member_access)]

use anyhow::Context;
use texture_atlas::{IMargins, TextureAtlas};

mod error;
mod fnt;
mod font;
mod sources;
mod texture_atlas;

fn main() -> anyhow::Result<()> {
    let mut sources = sources::Sources::new();

    let m5x7_id = sources
        .try_load_source("assets/m5x7.fnt")
        .with_context(|| format!("Failed to load 'm5x7.fnt'"))?;
    let font = font::Font::from_fnt(m5x7_id, &sources)?;

    let m5x7_color_id = sources
        .try_load_source("assets/m5x7-color.fnt")
        .with_context(|| format!("Failed to load 'm5x7-color.fnt'"))?;
    let font_color = font::Font::from_fnt(m5x7_color_id, &sources)?;

    let mut atlas = TextureAtlas::new(
        vec![Box::new(font), Box::new(font_color)],
        IMargins::uniform(0),
    );

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

#![feature(error_generic_member_access)]

use anyhow::Context;

mod error;
mod fnt;

const FONT_DATA: &'static str = include_str!("../assets/m5x7-errors.fnt");

fn main() -> anyhow::Result<()> {
    let _fnt_file =
        fnt::FntFile::try_parse(FONT_DATA).context("Failed parsing 'm5x7-errors.fnt` FNT file")?;

    Ok(())
}

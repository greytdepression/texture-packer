#![feature(error_generic_member_access)]

mod error;
mod fnt;
mod sources;

fn main() -> anyhow::Result<()> {
    let mut sources = sources::Sources::new();
    let _m5x7_id = sources.try_load_source("assets/m5x7.fnt")?;

    Ok(())
}

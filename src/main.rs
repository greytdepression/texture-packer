
mod fnt;

const FONT_DATA: &'static str = include_str!("../assets/m5x7.fnt");

fn main() {
    let _fnt_file = fnt::FntFile::try_parse(FONT_DATA)
        .unwrap();
}

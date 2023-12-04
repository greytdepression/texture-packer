use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct FntFile {
    // Info `info`
    pub info: FntInfo,

    // Common `common`
    pub common: FntCommon,

    // Pages `page`
    pub pages: Vec<FntPage>,

    // Chars `char`
    pub chars: Vec<FntChar>,
}

#[derive(Debug, Clone, Default)]
pub struct FntPage {
    pub id: u32,
    pub file: String,
}

#[derive(Debug, Clone, Default)]
pub struct FntChar {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub x_advance: i32,
    pub page: u32,
    pub chnl: u32,
}

#[derive(Debug, Clone, Default)]
pub struct FntInfo {
    pub face: String,
    pub size: i32,
    pub bold: u8,
    pub italic: u8,
    pub charset: String,
    pub unicode: u8,
    pub stretch_h: i32,
    pub smooth: u8,
    pub aa: u8,
    pub padding: [i32; 4],
    pub spacing: [i32; 2],
}

#[derive(Debug, Clone, Default)]
pub struct FntCommon {
    pub line_height: i32,
    pub base: i32,
    pub scale_w: i32,
    pub scale_h: i32,
    pub num_pages: u32,
    pub packed: u8,
}

//--------------------------------------------------
// Implementations
//--------------------------------------------------

impl FntPage {
    const KEYWORD: &'static str = "page";

    fn try_parse(line: &str) -> Result<Self, String> {
        let mut output = Self::default();

        parse_line(line, |lhs, rhs| {
            match lhs {
                "id" => output.id = parse(rhs)?,
                "file" => output.file = parse_string(rhs)?,
                _ => panic!("Unrecognized attribute '{lhs}' in fnt page declaration."),
            }

            Ok(())
        })?;

        Ok(output)
    }
}

impl FntChar {
    const KEYWORD: &'static str = "char";

    fn try_parse(line: &str) -> Result<Self, String> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "id" => output.id = parse(rhs)?,
                "x" => output.x = parse(rhs)?,
                "y" => output.y = parse(rhs)?,
                "width" => output.width = parse(rhs)?,
                "height" => output.height = parse(rhs)?,
                "xoffset" => output.x_offset = parse(rhs)?,
                "yoffset" => output.y_offset = parse(rhs)?,
                "xadvance" => output.x_advance = parse(rhs)?,
                "page" => output.page = parse(rhs)?,
                "chnl" => output.chnl = parse(rhs)?,
                _ => panic!("Unrecognized attribute '{lhs}' in fnt char declaration."),
            }

            Ok(())
        })?;

        Ok(output)
    }
}

impl FntInfo {
    const KEYWORD: &'static str = "info";

    fn try_parse(line: &str) -> Result<Self, String> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "face" => output.face = parse_string(rhs)?,
                "size" => output.size = parse(rhs)?,
                "bold" => output.bold = parse(rhs)?,
                "italic" => output.italic = parse(rhs)?,
                "charset" => output.charset = parse_string(rhs)?,
                "unicode" => output.unicode = parse(rhs)?,
                "stretchH" => output.stretch_h = parse(rhs)?,
                "smooth" => output.smooth = parse(rhs)?,
                "aa" => output.aa = parse(rhs)?,
                "padding" => output.padding = parse_array(rhs)?,
                "spacing" => output.spacing = parse_array(rhs)?,
                _ => panic!("Unrecognized attribute '{lhs}' in fnt info declaration."),
            }

            Ok(())
        })?;

        Ok(output)
    }
}

impl FntCommon {
    const KEYWORD: &'static str = "common";

    fn try_parse(line: &str) -> Result<Self, String> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "lineHeight" => output.line_height = parse(rhs)?,
                "base" => output.base = parse(rhs)?,
                "scaleW" => output.scale_w = parse(rhs)?,
                "scaleH" => output.scale_h = parse(rhs)?,
                "pages" => output.num_pages = parse(rhs)?,
                "packed" => output.packed = parse(rhs)?,
                _ => panic!("Unrecognized attribute '{lhs}' in fnt info declaration."),
            }

            Ok(())
        })?;

        Ok(output)
    }
}

impl FntFile {
    pub fn try_parse(file_contents: &str) -> Result<Self, String> {
        let mut output = Self::default();

        for line in file_contents.lines() {
            let (ident, data) = consume_until_space(line);

            match ident {
                FntInfo::KEYWORD => output.info = FntInfo::try_parse(data)?,
                FntCommon::KEYWORD => output.common = FntCommon::try_parse(data)?,
                FntPage::KEYWORD => output.pages.push(FntPage::try_parse(data)?),
                "chars" => {}, // ignore
                FntChar::KEYWORD => output.chars.push(FntChar::try_parse(data)?),
                "kernings" => {}, // ignore for now
                _ => eprintln!("fnt::FntFile: Unrecognized keyword '{ident}' in fnt declaration."),
            }
        }

        Ok(output)
    }
}

fn parse<T: FromStr>(rhs: &str) -> Result<T, String> {
    rhs.parse().map_err(|_| {
        format!(
            "fnt::parse(): Encountered error trying to parse `{rhs}` as {}.",
            std::any::type_name::<T>(),
        )
    })
}

fn parse_array<T: FromStr + Default + Copy, const N: usize>(mut rhs: &str) -> Result<[T; N], String> {
    let mut output = [T::default(); N];

    let original_str = rhs;

    let mut index = 0;
    while !rhs.is_empty() && index < N {
        let (val, next) = consume_until_pat(rhs, ',');
        rhs = next;

        output[index] = parse(val)?;
        index += 1;
    }

    if index != N || !rhs.is_empty() {
        return Err(format!("fnt::parse_array(): The string `{original_str}` does not have {N} values."))
    }

    Ok(output)
}

fn parse_line<F>(mut line: &str, mut callback: F) -> Result<(), String>
where F: FnMut(&str, &str) -> Result<(), String> {
    while !line.is_empty() {
        let (expr, next) = consume_until_space(line);
        line = next;

        let Some((lhs, rhs)) = try_split_equality(expr) else {
            continue;
        };

        callback(lhs, rhs)?;
    }

    Ok(())
}

fn parse_string(value: &str) -> Result<String, String> {
    Ok(
        value.strip_prefix('"')
            .ok_or(format!("fnt::parse_string(): String value needs to start with '\"', but `{value}` does not!"))?
            .strip_suffix('"')
            .ok_or(format!("parse_string(): String value needs to end with '\"', but `{value}` does not!"))?
            .to_string()
    )
}

fn consume_until_space(line: &str) -> (&str, &str) {
    if let Some(index) = line.find(' ') {
        return (&line[0..index], line[index+1..].trim_start());
    }

    (line, "")
}


fn consume_until_pat(line: &str, pat: char) -> (&str, &str) {
    if let Some(index) = line.find(pat) {
        return (&line[0..index], &line[index+1..]);
    }

    (line, "")
}

fn try_split_equality(expr: &str) -> Option<(&str, &str)> {
    let index = expr.find("=")?;

    if expr.len() == index + 1 {
        return None;
    }

    Some((&expr[0..index], &expr[index+1..]))
}
#[cfg(test)]
mod tests {
    use super::{consume_until_space, FntFile};

    #[test]
    fn test_consume_until_space() {
        let mut line = "char id=0       x=0    y=0    width=7    height=13   xoffset=-1   yoffset=-1";

        while !line.is_empty() {
            let (lhs, rhs) = consume_until_space(line);

            println!("`{}`", lhs);

            line = rhs;
        }
    }

    #[test]
    fn test_parse_file() -> Result<(), String> {
        let test_file = include_str!("../assets/m5x7.fnt");

        let fnt_file = FntFile::try_parse(test_file)?;

        dbg!(
            &fnt_file.info,
            &fnt_file.common,
            &fnt_file.pages,
            &fnt_file.chars[..5],
            fnt_file.chars.len(),
        );

        Ok(())
    }

}

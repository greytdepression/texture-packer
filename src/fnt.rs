use std::{fmt::Debug, str::FromStr};

use anyhow::Context;

use crate::error::Ewwow;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FntPage {
    pub id: u32,
    pub file: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

    fn try_parse(line: &str) -> anyhow::Result<Self> {
        let mut output = Self::default();

        parse_line(line, |lhs, rhs| {
            match lhs {
                "id" => output.id = parse(rhs).context("Failed parsing 'id' attribute")?,
                "file" => {
                    output.file = parse_string(rhs).context("Failed parsing 'file' attribute")?
                }
                _ => Ewwow
                    .raise()
                    .with_context(|| format!("Encountered unknown attribute `{lhs}`"))?,
            }

            Ok(())
        })
        .context("Failed parsing FNT page")?;

        Ok(output)
    }
}

impl FntChar {
    const KEYWORD: &'static str = "char";

    fn try_parse(line: &str) -> anyhow::Result<Self> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "id" => output.id = parse(rhs).context("Failed parsing 'id' attribute")?,
                "x" => output.x = parse(rhs).context("Failed parsing 'x' attribute")?,
                "y" => output.y = parse(rhs).context("Failed parsing 'y' attribute")?,
                "width" => output.width = parse(rhs).context("Failed parsing 'width' attribute")?,
                "height" => {
                    output.height = parse(rhs).context("Failed parsing 'height' attribute")?
                }
                "xoffset" => {
                    output.x_offset = parse(rhs).context("Failed parsing 'xoffset' attribute")?
                }
                "yoffset" => {
                    output.y_offset = parse(rhs).context("Failed parsing 'yoffset' attribute")?
                }
                "xadvance" => {
                    output.x_advance = parse(rhs).context("Failed parsing 'xadvance' attribute")?
                }
                "page" => output.page = parse(rhs).context("Failed parsing 'page' attribute")?,
                "chnl" => output.chnl = parse(rhs).context("Failed parsing 'chnl' attribute")?,
                _ => Ewwow
                    .raise()
                    .with_context(|| format!("Encountered unknown attribute `{lhs}`"))?,
            }

            Ok(())
        })
        .context("Failed parsing FNT char")?;

        Ok(output)
    }
}

impl FntInfo {
    const KEYWORD: &'static str = "info";

    fn try_parse(line: &str) -> anyhow::Result<Self> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "face" => {
                    output.face = parse_string(rhs).context("Failed parsing 'face' attribute")?
                }
                "size" => output.size = parse(rhs).context("Failed parsing 'size' attribute")?,
                "bold" => output.bold = parse(rhs).context("Failed parsing 'bold' attribute")?,
                "italic" => {
                    output.italic = parse(rhs).context("Failed parsing 'italic' attribute")?
                }
                "charset" => {
                    output.charset =
                        parse_string(rhs).context("Failed parsing 'charset' attribute")?
                }
                "unicode" => {
                    output.unicode = parse(rhs).context("Failed parsing 'unicode' attribute")?
                }
                "stretchH" => {
                    output.stretch_h = parse(rhs).context("Failed parsing 'stretchH' attribute")?
                }
                "smooth" => {
                    output.smooth = parse(rhs).context("Failed parsing 'smooth' attribute")?
                }
                "aa" => output.aa = parse(rhs).context("Failed parsing 'aa' attribute")?,
                "padding" => {
                    output.padding =
                        parse_array(rhs).context("Failed parsing 'padding' attribute")?
                }
                "spacing" => {
                    output.spacing =
                        parse_array(rhs).context("Failed parsing 'spacing' attribute")?
                }
                _ => Ewwow
                    .raise()
                    .with_context(|| format!("Encountered unknown attribute `{lhs}`"))?,
            }

            Ok(())
        })
        .context("Failed parsing FNT info")?;

        Ok(output)
    }
}

impl FntCommon {
    const KEYWORD: &'static str = "common";

    fn try_parse(line: &str) -> anyhow::Result<Self> {
        let mut output = Self::default();
        parse_line(line, |lhs, rhs| {
            match lhs {
                "lineHeight" => {
                    output.line_height =
                        parse(rhs).context("Failed parsing 'lineHeight' attribute")?
                }
                "base" => output.base = parse(rhs).context("Failed parsing 'base' attribute")?,
                "scaleW" => {
                    output.scale_w = parse(rhs).context("Failed parsing 'scaleW' attribute")?
                }
                "scaleH" => {
                    output.scale_h = parse(rhs).context("Failed parsing 'scaleH' attribute")?
                }
                "pages" => {
                    output.num_pages = parse(rhs).context("Failed parsing 'pages' attribute")?
                }
                "packed" => {
                    output.packed = parse(rhs).context("Failed parsing 'packed' attribute")?
                }
                _ => Ewwow
                    .raise()
                    .with_context(|| format!("Encountered unknown attribute `{lhs}`"))?,
            }

            Ok(())
        })
        .context("Failed parsing FNT common")?;

        Ok(output)
    }
}

impl FntFile {
    pub fn try_parse(file_contents: &str) -> anyhow::Result<Self> {
        let mut output = Self::default();

        for (num, line) in file_contents.lines().enumerate() {
            let (ident, data) = consume_until_space(line);

            let ctxt = || format!("Failed parsing line {}", num + 1);

            match ident {
                FntInfo::KEYWORD => output.info = FntInfo::try_parse(data).with_context(ctxt)?,
                FntCommon::KEYWORD => {
                    output.common = FntCommon::try_parse(data).with_context(ctxt)?
                }
                FntPage::KEYWORD => output
                    .pages
                    .push(FntPage::try_parse(data).with_context(ctxt)?),
                "chars" => {} // ignore
                FntChar::KEYWORD => output
                    .chars
                    .push(FntChar::try_parse(data).with_context(ctxt)?),
                "kernings" => {} // ignore for now
                _ => Ewwow
                    .raise()
                    .with_context(|| format!("Encountered unknown attribute `{ident}`"))?,
            }
        }

        Ok(output)
    }
}

fn parse<T: Debug + FromStr>(rhs: &str) -> anyhow::Result<T> {
    rhs.parse::<T>().map_err(|_| Ewwow).with_context(|| {
        format!(
            "Failed parsing literal `{rhs}` as {}",
            std::any::type_name::<T>()
        )
    })
}

fn parse_array<T: FromStr + Default + Copy + Debug, const N: usize>(
    mut rhs: &str,
) -> anyhow::Result<[T; N]> {
    let mut output = [T::default(); N];

    let original = rhs;

    let mut index = 0;
    while !rhs.is_empty() && index < N {
        let (val, next) = consume_until_pat(rhs, ',');
        rhs = next;

        output[index] = parse(val)?;
        index += 1;
    }

    if index != N || !rhs.is_empty() {
        Ewwow.raise().with_context(|| {
            format!(
                "Failed parsing literal `{original}` as array {}",
                std::any::type_name::<[T; N]>()
            )
        })?
    }

    Ok(output)
}

fn parse_line<F>(mut line: &str, mut callback: F) -> anyhow::Result<()>
where
    F: FnMut(&str, &str) -> anyhow::Result<()>,
{
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

fn parse_string(value: &str) -> anyhow::Result<String> {
    Ok(value
        .strip_prefix('"')
        .ok_or(Ewwow)
        .with_context(|| format!("Failed to parse `{value}` as a string: misses opening \""))?
        .strip_suffix('"')
        .ok_or(Ewwow)
        .with_context(|| format!("Failed to parse `{value}` as a string: misses closing \""))?
        .to_string())
}

fn consume_until_space(line: &str) -> (&str, &str) {
    if let Some(index) = line.find(' ') {
        return (&line[0..index], line[index + 1..].trim_start());
    }

    (line, "")
}

fn consume_until_pat(line: &str, pat: char) -> (&str, &str) {
    if let Some(index) = line.find(pat) {
        return (&line[0..index], &line[index + 1..]);
    }

    (line, "")
}

fn try_split_equality(expr: &str) -> Option<(&str, &str)> {
    let index = expr.find("=")?;

    if expr.len() == index + 1 {
        return None;
    }

    Some((&expr[0..index], &expr[index + 1..]))
}

#[cfg(test)]
mod tests {
    use super::{consume_until_space, FntFile};

    #[test]
    fn test_consume_until_space() {
        let line = "char id=0       x=0    y=0    width=7    height=13   xoffset=-1   yoffset=-1";

        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "char");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "id=0");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "x=0");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "y=0");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "width=7");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "height=13");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "xoffset=-1");
        let (lhs, line) = consume_until_space(line);
        assert_eq!(lhs, "yoffset=-1");

        assert!(line.is_empty());
    }

    #[test]
    fn test_parse_file() -> anyhow::Result<()> {
        let test_file = include_str!("../assets/m5x7.fnt");

        let _ = FntFile::try_parse(test_file)?;

        Ok(())
    }
}

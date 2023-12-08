use anyhow::Context;
use image::{GenericImage, RgbaImage};

use crate::{error::Ewwow, math::*, sources::Sources};

use super::font::FontIntermediate;

pub struct TextureAtlas {
    pub fonts: Vec<FontIntermediate>,
    pub sprite_sizes: Vec<(usize, usize, ISize)>,
    pub sprite_bounds: Vec<(usize, usize, IRect)>,
    pub padding: IMargins,
    pub final_image_bounds: ISize,
    image_side_len_guess: u32,
}

impl TextureAtlas {
    pub fn new(padding: IMargins) -> Self {
        Self {
            fonts: vec![],
            sprite_sizes: Vec::new(),
            sprite_bounds: Vec::new(),
            padding,
            final_image_bounds: ISize::default(),
            image_side_len_guess: 0,
        }
    }

    pub fn with_font(&mut self, font: FontIntermediate) {
        self.fonts.push(font);
    }

    pub fn load_sizes(&mut self) {
        self.sprite_sizes.clear();

        let mut area = 0;

        // Font asset indices start at 0
        let mut font_sizes: Vec<_> = self
            .fonts
            .iter()
            .enumerate()
            .flat_map(|(asset_index, font)| {
                font.get_sprite_sizes()
                    .iter()
                    .enumerate()
                    .map(|(sprite_index, size)| {
                        area += size.area();

                        (asset_index, sprite_index, *size)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // The next type of asset's asset indices start at `fonts.len()`

        self.sprite_sizes.append(&mut font_sizes);

        // Get a guess for what the size of the atlas should be
        let area_sqrt = (area as f32).sqrt();

        self.image_side_len_guess = (area_sqrt.ceil() as u32).next_power_of_two() / 2;

        println!(
            "Loaded {} sprite sizes. Guess for image side len is {}.",
            self.sprite_sizes.len(),
            self.image_side_len_guess,
        );
    }

    pub fn pack(&mut self) {
        let mut width = self.image_side_len_guess as i32;
        let mut height = self.image_side_len_guess as i32;

        loop {
            if width > 1024 {
                panic!("Not terminating");
            }

            if !self.try_pack(width, height) {
                if width == height {
                    width *= 2;
                } else {
                    height *= 2;
                }

                assert!(width >= height);
                continue;
            }

            self.final_image_bounds = ISize::new(width as i32, height as i32);

            println!("Final image size is {width}x{height}");

            break;
        }
    }

    fn get_asset_sprite_texture(
        &self,
        asset_id: usize,
        sprite_id: usize,
        srcs: &Sources,
    ) -> anyhow::Result<image::RgbaImage> {
        // The first `self.fonts.len()` asset ids refer to fonts
        if asset_id < self.fonts.len() {
            return self.fonts[asset_id]
                .get_sprite_texture(sprite_id, srcs)
                .with_context(|| {
                    format!(
                        "Failed to get sprite #{sprite_id} from font '{}'",
                        &self.fonts[asset_id].name
                    )
                });
        }

        Ewwow.raise()
            .with_context(|| format!("Failed to get sprite texture from asset #{asset_id} as this asset id does not exist"))?;

        unreachable!()
    }

    pub fn build_image(&self, srcs: &Sources) -> anyhow::Result<image::RgbaImage> {
        let mut output = RgbaImage::new(
            self.final_image_bounds.width as u32,
            self.final_image_bounds.height as u32,
        );

        for &(asset_id, sprite_id, bounds) in self.sprite_bounds.iter() {
            let sprite_texture = self
                .get_asset_sprite_texture(asset_id, sprite_id, srcs)
                .with_context(|| {
                    format!("Failed to retrieve sprite #{sprite_id} of asset #{asset_id}")
                })?;

            assert!(sprite_texture.width() == bounds.uwidth());
            assert!(sprite_texture.height() == bounds.uheight());

            let x = bounds.min.x as u32;
            let y = bounds.min.y as u32;

            output.copy_from(&sprite_texture, x, y).with_context(|| {
                format!("Failed to copy sprite #{sprite_id} of asset #{asset_id} into final image")
            })?;
        }

        Ok(output)
    }

    fn try_pack(&mut self, width: i32, height: i32) -> bool {
        self.sprite_bounds.clear();

        // Sort the sprites by height
        self.sprite_sizes
            .sort_by(|&(_, _, a_size), &(_, _, b_size)| {
                // Use reverse cmp to get decreasing heights
                b_size.height.cmp(&a_size.height)
            });

        let mut current_x: i32 = 0;
        let mut current_y: i32 = 0;
        let mut next_y: i32 = 0;

        let mut index = 0;

        while index < self.sprite_sizes.len() {
            let (i1, i2, size) = self.sprite_sizes[index];

            // Sanity check -- if we didn't check this we could get an endless loop
            if size.width > width {
                return false;
            }

            // Start of a new row
            if current_x == 0 {
                // Check that the sprites actually fit in the row
                if current_y + size.height + self.padding.vert() > height {
                    return false;
                }

                next_y = current_y + size.height + self.padding.vert();
            }

            // Check that this sprite still fits in the row
            if current_x + self.padding.hori() + size.width > width {
                current_x = 0;
                current_y = next_y;
                continue;
            }

            // The sprite fits!
            let bounds = IRect::new(
                current_x + self.padding.left,
                current_y + self.padding.top,
                size.width,
                size.height,
            );

            self.sprite_bounds.push((i1, i2, bounds));

            current_x += size.width + self.padding.hori();

            index += 1;
        }

        true
    }

    pub fn get_font_asset_id(&self, font_index: usize) -> usize {
        font_index
    }
}

pub trait Atlasable {
    fn get_sprite_sizes(&self) -> Vec<ISize>;
    fn get_sprite_texture(&self, index: usize, srcs: &Sources) -> anyhow::Result<image::RgbaImage>;
}

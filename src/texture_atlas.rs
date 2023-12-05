use std::collections::BTreeMap;

use anyhow::Context;
use glam::IVec2;
use image::{GenericImage, RgbaImage};
use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};

use crate::sources::Sources;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ISize {
    pub width: i32,
    pub height: i32,
}

impl ISize {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn area(self) -> i32 {
        self.width * self.height
    }

    pub fn grow(self, padding: IMargins) -> Self {
        Self::new(
            self.width + padding.left + padding.right,
            self.height + padding.top + padding.bottom,
        )
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IRect {
    min: IVec2,
    max: IVec2,
}

impl IRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        IRect {
            min: IVec2::new(x, y),
            max: IVec2::new(x + w, y + h),
        }
    }

    pub fn shrink(self, padding: IMargins) -> Self {
        Self {
            min: IVec2::new(self.min.x + padding.left, self.min.y + padding.top),
            max: IVec2::new(self.max.x - padding.right, self.max.y - padding.right),
        }
    }

    pub fn uwidth(self) -> u32 {
        (self.max.x - self.min.x) as u32
    }

    pub fn uheight(self) -> u32 {
        (self.max.y - self.min.y) as u32
    }
}

pub trait Atlasable {
    fn get_sprite_sizes(&self) -> Vec<ISize>;
    fn get_sprite_texture(&self, index: usize, srcs: &Sources) -> anyhow::Result<image::RgbaImage>;
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IMargins {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

impl IMargins {
    pub fn new(t: i32, b: i32, l: i32, r: i32) -> Self {
        Self {
            top: t,
            bottom: b,
            left: l,
            right: r,
        }
    }

    pub fn uniform(m: i32) -> Self {
        Self::new(m, m, m, m)
    }
}

pub struct TextureAtlas {
    pub assets: Vec<Box<dyn Atlasable>>,
    pub sprite_sizes: Vec<(usize, usize, ISize)>,
    pub sprite_bounds: Vec<(usize, usize, IRect)>,
    pub padding: IMargins,
    final_image_bounds: ISize,
    image_side_len_guess: u32,
}

impl TextureAtlas {
    pub fn new(assets: Vec<Box<dyn Atlasable>>, padding: IMargins) -> Self {
        Self {
            assets,
            sprite_sizes: Vec::new(),
            sprite_bounds: Vec::new(),
            padding,
            final_image_bounds: ISize::default(),
            image_side_len_guess: 0,
        }
    }

    pub fn load_sizes(&mut self) {
        let mut area = 0;

        self.sprite_sizes = self
            .assets
            .iter()
            .enumerate()
            .flat_map(|(asset_index, atl)| {
                atl.get_sprite_sizes()
                    .iter()
                    .enumerate()
                    .map(|(sprite_index, size)| {
                        area += size.area();

                        (asset_index, sprite_index, *size)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Get a guess for what the size of the atlas should be
        let area_sqrt = (area as f32).sqrt();

        self.image_side_len_guess = (area_sqrt.ceil() as u32).next_power_of_two();

        println!(
            "Loaded {} sprite sizes. Guess for image side len is {}.",
            self.sprite_sizes.len(),
            self.image_side_len_guess,
        );
    }

    pub fn pack(&mut self) {
        let mut rects_to_place = GroupedRectsToPlace::<_, i32>::new();

        for &(asset_id, sprite_id, size) in self.sprite_sizes.iter() {
            let size = size.grow(self.padding);

            rects_to_place.push_rect(
                (asset_id, sprite_id),
                None,
                RectToInsert::new(size.width as u32, size.height as u32, 1),
            )
        }

        let mut width = self.image_side_len_guess;
        let mut height = self.image_side_len_guess;

        loop {
            if width > 1024 {
                panic!("Not terminating");
            }

            let mut target_bin = BTreeMap::new();
            target_bin.insert(0, TargetBin::new(width, height, 1));

            let placements = pack_rects(
                &rects_to_place,
                &mut target_bin,
                &volume_heuristic,
                &contains_smallest_box,
            );

            if placements.is_err() {
                // rectangle-pack seems to prefer long images
                if width == height {
                    height *= 2;
                } else {
                    width *= 2;
                }

                assert!(width <= height);
                continue;
            }

            let Ok(placements) = placements else {
                unreachable!();
            };

            println!("Successfully placed rects in atlas of size {width}x{height}");

            self.final_image_bounds = ISize::new(width as i32, height as i32);

            for (&(asset_id, sprite_id), &(_, loc)) in placements.packed_locations().iter() {
                self.sprite_bounds.push((
                    asset_id,
                    sprite_id,
                    IRect::new(
                        loc.x() as i32,
                        loc.y() as i32,
                        loc.width() as i32,
                        loc.height() as i32,
                    )
                    .shrink(self.padding),
                ));
            }

            break;
        }
    }

    pub fn build_image(&self, srcs: &Sources) -> anyhow::Result<image::RgbaImage> {
        let mut output = RgbaImage::new(
            self.final_image_bounds.width as u32,
            self.final_image_bounds.height as u32,
        );

        for &(asset_id, sprite_id, bounds) in self.sprite_bounds.iter() {
            let sprite_texture = self.assets[asset_id]
                .get_sprite_texture(sprite_id, srcs)
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
}

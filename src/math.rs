use glam::IVec2;

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
    pub min: IVec2,
    pub max: IVec2,
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

    pub fn width(self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> i32 {
        self.max.y - self.min.y
    }

    pub fn uwidth(self) -> u32 {
        (self.max.x - self.min.x) as u32
    }

    pub fn uheight(self) -> u32 {
        (self.max.y - self.min.y) as u32
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IMargins {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
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

    pub fn vert(self) -> i32 {
        self.top + self.bottom
    }

    pub fn hori(self) -> i32 {
        self.left + self.right
    }
}

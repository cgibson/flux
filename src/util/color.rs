pub struct RGB {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl RGB {
    pub fn new(r: i32, g: i32, b: i32) -> RGB {
        RGB {r: r, g: g, b: b}
    }
}

pub struct ColorProfile {
    pub gamma: f64,
}

trait ImageColor {
    fn to_rgb(&self, profile: Option<ColorProfile>) -> RGB;
}

pub struct Spectrum {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ImageColor for Spectrum {
    fn to_rgb(&self, profile: Option<ColorProfile>) -> RGB {
        let col_space = |f: f64| (f * 256.0) as i32;
        match profile {
            Some(p) => RGB::new(col_space(self.r.powf(p.gamma)),
                                col_space(self.g.powf(p.gamma)),
                                col_space(self.b.powf(p.gamma))),
            None => RGB::new(col_space(self.r),
                             col_space(self.g),
                             col_space(self.b))
        }
    }
}

pub struct Pixel {
    pub x: i32,
    pub y: i32,
}
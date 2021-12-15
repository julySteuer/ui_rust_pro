#[derive(Debug, Clone)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

pub fn rgb_2_int(color:&Color) -> u32 {
    (color.r << 16) + (color.g << 8) + color.b
}

pub fn int_2_rgb(color:u32) -> Color {
    Color{
        r: (color & 0xff0000) >> 16,
        g: (color & 0x00ff00) >> 8,
        b: (color & 0x0000ff)
    }
}
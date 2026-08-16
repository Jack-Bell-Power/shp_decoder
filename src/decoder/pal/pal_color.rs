use image::Rgba;

#[derive(Clone, Copy)]
pub struct PalColor {
    /// red 5 bits
    pub red: u8,
    /// green 6 bits
    pub green: u8,
    /// blue 5 bits
    pub blue: u8,
}

impl From<PalColor> for Rgba<u8> {
    fn from(value: PalColor) -> Self {
        Rgba([
            ((value.red as u32 * 255) / 63) as u8,
            ((value.green as u32 * 255) / 63) as u8,
            ((value.blue as u32 * 255) / 63) as u8,
            255,
        ])
    }
}

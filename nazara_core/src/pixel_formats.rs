use crate::enums::{PixelFormatContent, PixelFormatType};

#[derive(Debug)]
pub struct PixelFormatInfo {
    pub bits_per_pixel: u8,
    pub content: PixelFormatContent,
    pub name: &'static str,
    pub red_bitmask: u128,
    pub green_bitmask: u128,
    pub blue_bitmask: u128,
    pub alpha_bitmask: u128,
}

impl PixelFormatInfo {
    fn new(
        name: &'static str,
        bpp: u8,
        content: PixelFormatContent,
        red_bitmask: u128,
        green_bitmask: u128,
        blue_bitmask: u128,
        alpha_bitmask: u128,
    ) -> PixelFormatInfo {
        PixelFormatInfo {
            bits_per_pixel: bpp,
            content,
            name,
            red_bitmask,
            green_bitmask,
            blue_bitmask,
            alpha_bitmask,
        }
    }
}

impl PixelFormatType {
    /// Returns how many bytes per pixel this format uses to store X pixels
    pub fn compute_size(self, pixel_count: usize) -> usize {
        (self.info().bits_per_pixel as usize * pixel_count + 7) / 8
    }

    /// Returns informations about a pixel format
    pub fn info(self) -> PixelFormatInfo {
        // TODO make it static
        match self {
            PixelFormatType::RGB8 => PixelFormatInfo::new(
                "RGB8",
                24,
                PixelFormatContent::ColorRGBA,
                0xFF00_0000,
                0x00FF_0000,
                0x0000_FF00,
                0,
            ),
            _ => PixelFormatInfo::new("TODO", 0, PixelFormatContent::ColorRGBA, 0, 0, 0, 0),
        }
    }
}

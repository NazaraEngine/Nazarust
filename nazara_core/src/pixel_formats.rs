use crate::enums::{PixelFormatContent, PixelFormatType};

/// Store a pixel format info
#[derive(Debug, PartialEq)]
pub struct PixelFormatInfo {
    /// Number of bits required to store one pixel
    pub bits_per_pixel: u8,
    /// Pixel content format
    pub content: PixelFormatContent,
    /// Pixel format name
    pub name: &'static str,
    /// Bitmask for red color
    pub red_bitmask: u128,
    /// Bitmask for gree color
    pub green_bitmask: u128,
    /// Bitmask for blue color
    pub blue_bitmask: u128,
    /// Bitmask for alpha channel
    pub alpha_bitmask: u128,
}

impl PixelFormatInfo {
    /// Create a new pixel format
    ///
    /// # Arguments
    /// * `name` - Name of pixel format
    /// * `bpp` - Number of bits required to store one pixel
    /// * `content` - Pixel format content
    /// * `red_bitmask` - Bitmask for red color
    /// * `green_bitmask` - Bitmask for green color
    /// * `blue_bitmask` - Bitmask for blue color
    /// * `alpha_bitmask` - Bitmask for alpha color
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
    ///
    /// ```
    /// use nazara_core::pixel_formats::PixelFormatInfo;
    /// use nazara_core::enums::{PixelFormatContent, PixelFormatType};
    /// let pixel_info = PixelFormatInfo {
    ///     bits_per_pixel: 24,
    ///     content: PixelFormatContent::ColorRGBA,
    ///     name: "RGB8",
    ///     red_bitmask: 0xFF00_0000,
    ///     green_bitmask: 0x00FF_0000,
    ///     blue_bitmask: 0x0000_FF00,
    ///     alpha_bitmask: 0x0000_0000,
    /// };
    /// let pixel_format_type = PixelFormatType::RGB8;
    ///
    /// assert_eq!(pixel_format_type.compute_size(1234), 1234*3);
    /// assert_eq!(pixel_format_type.compute_size(0), 0);
    /// ```
    ///
    /// # Arguments
    /// * `pixel_count` - Number of pixel
    pub fn compute_size(self, pixel_count: usize) -> usize {
        (self.info().bits_per_pixel as usize * pixel_count + 7) / 8
    }

    /// Returns informations about a pixel format
    ///
    /// ```
    /// use nazara_core::enums::{PixelFormatType, PixelFormatContent};
    /// use nazara_core::pixel_formats::PixelFormatInfo;
    /// assert_eq!(PixelFormatType::RGB8.info(), PixelFormatInfo {
    ///     bits_per_pixel: 24,
    ///     content: PixelFormatContent::ColorRGBA,
    ///     name: "RGB8",
    ///     red_bitmask: 0xFF00_0000,
    ///     green_bitmask: 0x00FF_0000,
    ///     blue_bitmask: 0x0000_FF00,
    ///     alpha_bitmask: 0x0000_0000,
    /// });
    /// assert_eq!(PixelFormatType::RGBA8.info(), PixelFormatInfo {
    ///     bits_per_pixel: 32,
    ///     content: PixelFormatContent::ColorRGBA,
    ///     name: "RGBA8",
    ///     red_bitmask: 0xFF00_0000,
    ///     green_bitmask: 0x00FF_0000,
    ///     blue_bitmask: 0x0000_FF00,
    ///     alpha_bitmask: 0x0000_00FF,
    /// });
    /// ```
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
            PixelFormatType::RGBA8 => PixelFormatInfo::new(
                "RGBA8",
                32,
                PixelFormatContent::ColorRGBA,
                0xFF00_0000,
                0x00FF_0000,
                0x0000_FF00,
                0x0000_00FF,
            ),
            _ => PixelFormatInfo::new("TODO", 0, PixelFormatContent::ColorRGBA, 0, 0, 0, 0),
        }
    }
}

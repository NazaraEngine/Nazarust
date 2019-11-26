use crate::enums::{ImageType, PixelFormatType};
use cgmath::Vector3;

pub struct Image {
    dimensions: Vector3<usize>,
    content: Vec<Vec<u8>>,
    image_type: ImageType,
    pixel_format: PixelFormatType,
}

impl Image {
    pub fn new_1d(format: PixelFormatType, width: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, 1, 1),
            content: vec![vec![0u8; width]; 1],
            image_type: ImageType::Single1D,
            pixel_format: format,
        }
    }

    pub fn new_2d(format: PixelFormatType, width: usize, height: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, height, 1),
            content: vec![vec![0u8; width * height]; 1],
            image_type: ImageType::Single2D,
            pixel_format: format,
        }
    }

    pub fn new_3d(format: PixelFormatType, width: usize, height: usize, depth: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, height, depth),
            content: vec![vec![0u8; width * height * depth]; 1],
            image_type: ImageType::Single3D,
            pixel_format: format,
        }
    }

    pub fn get_image_type(&self) -> ImageType {
        self.image_type
    }

    pub fn get_pixel_format(&self) -> PixelFormatType {
        self.pixel_format
    }

    pub fn get_size(&self) -> Vector3<usize> {
        self.dimensions
    }
}

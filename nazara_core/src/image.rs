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

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use crate::enums::{PixelFormatType, ImageType};
    use cgmath::Vector3;

    #[test]
    fn test_new_1d () {
        let new_1d = Image::new_1d(PixelFormatType::A8, 50);
        assert_eq!(new_1d.get_size(), Vector3 { x: 50, y: 1, z: 1 });
        assert_eq!(new_1d.get_pixel_format(), PixelFormatType::A8);
        assert_eq!(new_1d.get_image_type(), ImageType::Single1D);
    }

    #[test]
    fn test_new_2d () {
        let new_2d = Image::new_2d(PixelFormatType::A8, 50, 60);
        assert_eq!(new_2d.get_size(), Vector3 { x: 50, y: 60, z: 1 });
        assert_eq!(new_2d.get_pixel_format(), PixelFormatType::A8);
        assert_eq!(new_2d.get_image_type(), ImageType::Single2D);
    }

    #[test]
    fn test_new_3d () {
        let new_3d = Image::new_3d(PixelFormatType::A8, 50, 60, 70);
        assert_eq!(new_3d.get_size(), Vector3 { x: 50, y: 60, z: 70 });
        assert_eq!(new_3d.get_pixel_format(), PixelFormatType::A8);
        assert_eq!(new_3d.get_image_type(), ImageType::Single3D);
    }
}
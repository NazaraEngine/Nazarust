use crate::enums::{ImageType, PixelFormatType};
use cgmath::Vector3;

/// Image structure for Nazarust
///
/// Mettre ici une description plus avancé, j'ai pas encore compris à quoi ça sert
pub struct Image {
    dimensions: Vector3<usize>,
    content: Vec<Vec<u8>>,
    image_type: ImageType,
    pixel_format: PixelFormatType,
}

impl Image {
    /// Creates a new [`ImageType::Single1D`] [`Image`] with the given pixel format and width.
    ///
    /// You can create a new [`ImageType::Single1D`] image:
    ///
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let image = Image::new_1d(PixelFormatType::A8, 40);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    ///
    /// [`ImageType::Single1D`]: crate::enums::ImageType::Single1D
    pub fn new_1d(format: PixelFormatType, width: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, 1, 1),
            content: vec![vec![0u8; width]; 1],
            image_type: ImageType::Single1D,
            pixel_format: format,
        }
    }

    /// Creates a new [`ImageType::Single2D`] [`Image`] with the given pixel format, width and height.
    ///
    /// You can create a new [`ImageType::Single2D`] image:
    ///
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let image = Image::new_2d(PixelFormatType::A8, 40, 50);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    ///
    /// [`ImageType::Single2D`]: crate::enums::ImageType::Single2D
    pub fn new_2d(format: PixelFormatType, width: usize, height: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, height, 1),
            content: vec![vec![0u8; width * height]; 1],
            image_type: ImageType::Single2D,
            pixel_format: format,
        }
    }

    /// Creates a new [`ImageType::Single3D`] [`Image`] with the given pixel format, width, height and depth.
    ///
    /// You can create a new [`ImageType::Single3D`] image:
    ///
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let image = Image::new_3d(PixelFormatType::A8, 40, 50, 60);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    ///
    /// [`ImageType::Single3D`]: crate::enums::ImageType::Single3D
    pub fn new_3d(format: PixelFormatType, width: usize, height: usize, depth: usize) -> Image {
        Image {
            dimensions: Vector3::new(width, height, depth),
            content: vec![vec![0u8; width * height * depth]; 1],
            image_type: ImageType::Single3D,
            pixel_format: format,
        }
    }

    /// Return image type of an [`Image`] instance.
    ///
    /// You can get image type from any [`Image`] instance:
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let img_1d = Image::new_1d(PixelFormatType::A8, 40);
    /// let img_2d = Image::new_2d(PixelFormatType::A8, 40, 50);
    /// let img_3d = Image::new_3d(PixelFormatType::A8, 40, 50, 60);
    /// let img_1d_type = img_1d.get_image_type(); // img_1d_type is ImageType::Single1D
    /// let img_2d_type = img_2d.get_image_type(); // img_2d_type is ImageType::Single2D
    /// let img_3d_type = img_3d.get_image_type(); // img_3d_type is ImageType::Single3D
    /// ```
    pub fn get_image_type(&self) -> ImageType {
        self.image_type
    }

    /// Return pixel format of an [`Image`] instance.
    ///
    /// You can get pixel type from any [`Image`] instance:
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let img_1d = Image::new_1d(PixelFormatType::A8, 40);
    /// let img_2d = Image::new_2d(PixelFormatType::R16F, 40, 50);
    /// let img_3d = Image::new_3d(PixelFormatType::RGB16F, 40, 50, 60);
    /// let img_1d_pixel_type = img_1d.get_image_type(); // img_1d_type is PixelFormatType::A8
    /// let img_2d_pixel_type = img_2d.get_image_type(); // img_2d_type is PixelFormatType::R16F
    /// let img_3d_pixel_type = img_3d.get_image_type(); // img_3d_type is PixelFormatType::RGB16F
    /// ```
    pub fn get_pixel_format(&self) -> PixelFormatType {
        self.pixel_format
    }

    /// Return size of an [`Image`] instance.
    ///
    /// You can get size from any [`Image`] instance:
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::PixelFormatType;
    ///
    /// let img_1d = Image::new_1d(PixelFormatType::A8, 40);
    /// let img_2d = Image::new_2d(PixelFormatType::R16F, 40, 50);
    /// let img_3d = Image::new_3d(PixelFormatType::RGB16F, 40, 50, 60);
    /// let img_1d_size = img_1d.get_size(); // img_1d_size is Vector3 { x: 40, y: 1, z: 1 }
    /// let img_2d_size = img_2d.get_size(); // img_2d_size is Vector3 { x: 40, y: 50, z: 1 }
    /// let img_3d_size = img_3d.get_size(); // img_3d_size is Vector3 { x: 40, y: 50, z: 60 }
    /// ```
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
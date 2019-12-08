use crate::enums::{ImageType, PixelFormatType};
use cgmath::Vector3;
use image::{io::Reader, DynamicImage, GenericImageView};
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Seek},
    path::Path,
};

/// Image structure for Nazarust
///
/// Mettre ici une description plus avancé, j'ai pas encore compris à quoi ça sert
pub struct Image {
    dimensions: Vector3<usize>,
    #[allow(dead_code)]
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
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    ///
    /// let image = Image::new_1d(PixelFormatType::A8, 40);
    /// assert_eq!(image.get_size(), Vector3 { x: 40, y: 1, z: 1 });
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::A8);
    /// assert_eq!(image.get_image_type(), ImageType::Single1D);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    ///
    /// [`ImageType::Single1D`]: crate::enums::ImageType::Single1D
    pub fn new_1d(format: PixelFormatType, width: usize) -> Image {
        let size = PixelFormatType::compute_size(format, width);

        Image {
            dimensions: Vector3::new(width, 1, 1),
            content: vec![vec![0u8; size]; 1],
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
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    ///
    /// let image = Image::new_2d(PixelFormatType::A8, 40, 50);
    /// assert_eq!(image.get_size(), Vector3 { x: 40, y: 50, z: 1 });
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::A8);
    /// assert_eq!(image.get_image_type(), ImageType::Single2D);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    /// * `height` - Height for new image
    ///
    /// [`ImageType::Single2D`]: crate::enums::ImageType::Single2D
    pub fn new_2d(format: PixelFormatType, width: usize, height: usize) -> Image {
        let size = PixelFormatType::compute_size(format, width * height);

        Image {
            dimensions: Vector3::new(width, height, 1),
            content: vec![vec![0u8; size]; 1],
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
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    ///
    /// let image = Image::new_3d(PixelFormatType::A8, 40, 50, 60);
    /// assert_eq!(image.get_size(), Vector3 { x: 40, y: 50, z: 60 });
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::A8);
    /// assert_eq!(image.get_image_type(), ImageType::Single3D);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `format` - Format for stored pixels
    /// * `width` - Width for the new image
    /// * `height` - Height for the new image
    /// * `depth` - Depth of new image
    ///
    /// [`ImageType::Single3D`]: crate::enums::ImageType::Single3D
    pub fn new_3d(format: PixelFormatType, width: usize, height: usize, depth: usize) -> Image {
        let size = PixelFormatType::compute_size(format, width * height * depth);

        Image {
            dimensions: Vector3::new(width, height, depth),
            content: vec![vec![0u8; size]; 1],
            image_type: ImageType::Single3D,
            pixel_format: format,
        }
    }

    /// Return image type of an [`Image`] instance.
    ///
    /// You can get image type from any [`Image`] instance:
    /// ```
    /// use nazara_core::image::Image;
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    ///
    /// let img_1d = Image::new_1d(PixelFormatType::A8, 40);
    /// let img_2d = Image::new_2d(PixelFormatType::A8, 40, 50);
    /// let img_3d = Image::new_3d(PixelFormatType::A8, 40, 50, 60);
    /// let img_1d_type = img_1d.get_image_type(); // img_1d_type is ImageType::Single1D
    /// let img_2d_type = img_2d.get_image_type(); // img_2d_type is ImageType::Single2D
    /// let img_3d_type = img_3d.get_image_type(); // img_3d_type is ImageType::Single3D
    ///
    /// assert_eq!(img_1d_type, ImageType::Single1D);
    /// assert_eq!(img_2d_type, ImageType::Single2D);
    /// assert_eq!(img_3d_type, ImageType::Single3D);
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
    /// let img_1d_pixel_type = img_1d.get_pixel_format(); // img_1d_type is PixelFormatType::A8
    /// let img_2d_pixel_type = img_2d.get_pixel_format(); // img_2d_type is PixelFormatType::R16F
    /// let img_3d_pixel_type = img_3d.get_pixel_format(); // img_3d_type is PixelFormatType::RGB16F
    ///
    /// assert_eq!(img_1d_pixel_type, PixelFormatType::A8);
    /// assert_eq!(img_2d_pixel_type, PixelFormatType::R16F);
    /// assert_eq!(img_3d_pixel_type, PixelFormatType::RGB16F);
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
    /// use cgmath::Vector3;
    ///
    /// let img_1d = Image::new_1d(PixelFormatType::A8, 40);
    /// let img_2d = Image::new_2d(PixelFormatType::R16F, 40, 50);
    /// let img_3d = Image::new_3d(PixelFormatType::RGB16F, 40, 50, 60);
    /// let img_1d_size = img_1d.get_size(); // img_1d_size is Vector3 { x: 40, y: 1, z: 1 }
    /// let img_2d_size = img_2d.get_size(); // img_2d_size is Vector3 { x: 40, y: 50, z: 1 }
    /// let img_3d_size = img_3d.get_size(); // img_3d_size is Vector3 { x: 40, y: 50, z: 60 }
    ///
    /// assert_eq!(img_1d_size, Vector3 { x: 40, y: 1, z: 1 });
    /// assert_eq!(img_2d_size, Vector3 { x: 40, y: 50, z: 1 });
    /// assert_eq!(img_3d_size, Vector3 { x: 40, y: 50, z: 60 });
    /// ```
    pub fn get_size(&self) -> Vector3<usize> {
        self.dimensions
    }

    pub fn set_content(&mut self, new_content: Vec<Vec<u8>>) {
        self.content = new_content;
    }
}

/// Image loader for Nazarust
///
///
pub struct ImageLoader {}

impl ImageLoader {
    /// Load an image from file
    ///
    /// # Example
    /// ```
    /// use nazara_core::image::{ImageLoader, Image};
    /// use std::path::Path;
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    /// let image: Image = ImageLoader::load_from_file(Path::new("./test_ressources/image.png"));
    ///
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::RGB8);
    /// assert_eq!(image.get_image_type(), ImageType::Single2D);
    /// assert_eq!(image.get_size(), Vector3 { x: 800, y: 629, z:1 });
    /// ```
    ///
    /// # Arguments
    /// * `file` - [`std::path::Path`] of file to load
    pub fn load_from_file(file: &Path) -> Image {
        ImageLoader::load_from_reader(BufReader::new(File::open(file).unwrap()))
    }

    /// Load an image from memory
    ///
    /// # Example
    /// ```
    /// use std::fs;
    /// use nazara_core::image::{Image, ImageLoader};
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    /// let image: Image = ImageLoader::load_from_mem(&fs::read("./test_ressources/image.png").unwrap());
    ///
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::RGB8);
    /// assert_eq!(image.get_image_type(), ImageType::Single2D);
    /// assert_eq!(image.get_size(), Vector3 { x: 800, y:629, z:1 });
    /// ```
    ///
    /// # Arguments
    /// * `image` - Array of image content
    pub fn load_from_mem(image: &[u8]) -> Image {
        ImageLoader::load_from_reader(Cursor::new(image))
    }

    /// Load an image from stream ([`std::io::BufRead`], [`std::io::Seek`])
    ///
    /// # Load a png image
    /// ```
    /// use std::fs::File;
    /// use nazara_core::image::{ImageLoader, Image};
    /// use std::io::BufReader;
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    ///
    /// let file = File::open("./test_ressources/image.png").unwrap();
    /// let buf = BufReader::new(file);
    ///
    /// let image: Image = ImageLoader::load_from_reader(buf);
    ///
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::RGB8);
    /// assert_eq!(image.get_image_type(), ImageType::Single2D);
    /// assert_eq!(image.get_size(), Vector3 { x: 800, y:629, z:1 });
    /// ```
    ///
    /// # Load gif image
    /// ```
    /// use std::fs::File;
    /// use nazara_core::image::{ImageLoader, Image};
    /// use std::io::BufReader;
    /// use nazara_core::enums::{PixelFormatType, ImageType};
    /// use cgmath::Vector3;
    ///
    /// let file = File::open("./test_ressources/image.gif").unwrap();
    /// let buf = BufReader::new(file);
    ///
    /// let image: Image = ImageLoader::load_from_reader(buf);
    ///
    /// assert_eq!(image.get_pixel_format(), PixelFormatType::RGBA8);
    /// assert_eq!(image.get_image_type(), ImageType::Single2D);
    /// assert_eq!(image.get_size(), Vector3 { x: 800, y:629, z:1 });
    /// ```
    ///
    /// # Arguments
    ///
    /// * `reader` - Reader instance from which image will be loaded
    pub fn load_from_reader<R: BufRead + Seek>(reader: R) -> Image {
        let reader = Reader::new(reader).with_guessed_format().unwrap();

        let image = reader.decode().expect("Fail");
        let dimensions = image.dimensions();
        let pixels = vec![image.raw_pixels()];
        let color_type = match image {
            DynamicImage::ImageLuma8(_) => PixelFormatType::L8,
            DynamicImage::ImageLumaA8(_) => PixelFormatType::LA8,
            DynamicImage::ImageRgb8(_) => PixelFormatType::RGB8,
            DynamicImage::ImageRgba8(_) => PixelFormatType::RGBA8,
            DynamicImage::ImageBgr8(_) => PixelFormatType::BGR8,
            DynamicImage::ImageBgra8(_) => PixelFormatType::BGRA8,
        };
        let mut new_image = Image::new_2d(color_type, dimensions.0 as usize, dimensions.1 as usize);
        new_image.set_content(pixels);
        new_image
    }
}

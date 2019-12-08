use crate::enums::PixelFormatType;
use std::{error::Error, fmt, io};

pub type NazaraResult<T> = Result<T, NazaraError>;

#[derive(Debug)]
pub enum NazaraError {
    ImageError(ImageError),
}

impl fmt::Display for NazaraError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            NazaraError::ImageError(ref e) => e.fmt(fmt),
        }
    }
}

impl Error for NazaraError {
    fn description(&self) -> &str {
        match *self {
            NazaraError::ImageError(_) => "Image error",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            NazaraError::ImageError(ref e) => Some(e),
        }
    }
}

impl From<ImageError> for NazaraError {
    fn from(err: ImageError) -> Self {
        NazaraError::ImageError(err)
    }
}

#[derive(Debug)]
pub enum ImageError {
    /// The Image is not formatted properly
    FormatError(String),

    /// The Image's dimensions are either too small or too large
    DimensionError,

    /// The Decoder does not support this image format
    UnsupportedError(String),

    /// The Decoder does not support this color type
    UnsupportedPixelFormat(PixelFormatType),

    /// Not enough data was provided to the Decoder
    /// to decode the image
    NotEnoughData,

    /// An I/O Error occurred while decoding the image
    IoError(io::Error),

    /// The end of the image has been reached
    ImageEnd,

    /// There is not enough memory to complete the given operation
    InsufficientMemory,
}

impl fmt::Display for ImageError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ImageError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            ImageError::DimensionError => write!(
                fmt,
                "The Image's dimensions are either too \
                 small or too large"
            ),
            ImageError::IoError(ref e) => e.fmt(fmt),
            ImageError::UnsupportedError(ref f) => write!(
                fmt,
                "The Decoder does not support the \
                 image format `{}`",
                f,
            ),
            ImageError::InsufficientMemory => write!(fmt, "Insufficient memory"),
            ImageError::UnsupportedPixelFormat(ref c) => write!(
                fmt,
                "The decoder does not support \
                 the color type `{:?}`",
                c
            ),
            ImageError::NotEnoughData => write!(
                fmt,
                "Not enough data was provided to the \
                 Decoder to decode the image"
            ),
            ImageError::ImageEnd => write!(fmt, "The end of the image has been reached"),
        }
    }
}

impl Error for ImageError {
    fn description(&self) -> &str {
        match *self {
            ImageError::FormatError(..) => "Format error",
            ImageError::DimensionError => "Dimension error",
            ImageError::IoError(..) => "IO error",
            ImageError::UnsupportedError(_) => "Unsupported error",
            ImageError::InsufficientMemory => "Insufficient memory",
            ImageError::UnsupportedPixelFormat(_) => "Unsupported pixel format",
            ImageError::NotEnoughData => "Not enought data",
            ImageError::ImageEnd => "Image end",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ImageError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

impl From<image::ImageError> for ImageError {
    fn from(err: image::ImageError) -> Self {
        match err {
            image::ImageError::FormatError(s) => ImageError::FormatError(s),
            image::ImageError::DimensionError => ImageError::DimensionError,
            image::ImageError::UnsupportedError(s) => ImageError::UnsupportedError(s),
            image::ImageError::UnsupportedColor(c) => {
                ImageError::UnsupportedPixelFormat(PixelFormatType::from(c))
            }
            image::ImageError::NotEnoughData => ImageError::NotEnoughData,
            image::ImageError::IoError(e) => ImageError::IoError(e),
            image::ImageError::ImageEnd => ImageError::ImageEnd,
            image::ImageError::InsufficientMemory => ImageError::InsufficientMemory,
        }
    }
}

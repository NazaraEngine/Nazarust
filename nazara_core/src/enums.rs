#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents an image format
pub enum ImageType {
    /// TODO
    Array1D,
    /// TODO
    Array2D,
    /// TODO
    Cubemap,
    /// 1 dimensional image type
    Single1D,
    /// 2 dimensional image type
    Single2D,
    /// 3 dimensional image type
    Single3D,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents a pixel format type.
pub enum PixelFormatType {
    /// 8 bits grey pixel
    ///
    /// Stored in 1 uint8
    A8,
    /// 24 bits BGR pixel
    ///
    /// Stored in 3 uint8
    BGR8,
    /// 32 bits BGRA pixel
    ///
    /// Stored in 4 uint8
    BGRA8,
    ///
    ///
    /// Stored in
    DXT1,
    ///
    ///
    /// Stored in
    DXT3,
    ///
    ///
    /// Stored in
    DXT5,
    /// 8 bits luminance pixel
    ///
    /// Stored in 1 uint8
    L8,
    /// 16 bits luminance with alpha pixel
    ///
    /// Stored in 2 uint8
    LA8,
    /// 8 bits red only pixel
    ///
    /// Stored in 1 uint8
    R8,
    ///
    ///
    /// Stored in 1 int8
    R8I,
    ///
    ///
    /// Stored in 1 uint8
    R8UI,
    /// 16 bits red only pixel
    ///
    /// Stored in 1 uint16
    R16,
    ///
    ///
    /// Stored in 1 half
    R16F,
    ///
    ///
    /// Stored in 1 int16
    R16I,
    ///
    ///
    /// Stored in 1 uint16
    R16UI,
    ///
    ///
    /// Stored in 1 float
    R32F,
    ///
    ///
    /// Stored in 1 uint16
    R32I,
    ///
    ///
    /// Stored in 1 uint32
    R32UI,
    /// 16 bits red/green pixel
    ///
    /// Stored in 2 int8
    RG8,
    ///
    ///
    /// Stored in 2 int8
    RG8I,
    ///
    ///
    /// Stored in 2 uint8
    RG8UI,
    /// 32 bits red/green pixel
    ///
    /// Stored in 2 uint16
    RG16,
    ///
    ///
    /// Stored in 2 half
    RG16F,
    ///
    ///
    /// Stored in 2 int16
    RG16I,
    ///
    ///
    /// Stored in 2 uint16
    RG16UI,
    ///
    ///
    /// Stored in 2 float
    RG32F,
    ///
    ///
    /// Stored in 2 uint16
    RG32I,
    ///
    ///
    /// Stored in 2 uint32
    RG32UI,   // 2*uint32
    /// 16-bits RGBA pixel
    ///
    /// Stored in 3 uint5 and 1 alpha bit
    RGB5A1,
    /// 24 bits RGB pixel
    ///
    /// Stored in 3 uint8
    RGB8,
    ///
    ///
    /// Stored in 3 half
    RGB16F,
    ///
    ///
    /// Stored in 4 int16
    RGB16I,   // 4*int16
    ///
    ///
    /// Stored in 4 uint16
    RGB16UI,
    ///
    ///
    /// Stored in 3 float
    RGB32F,
    ///
    ///
    /// Stored in 4 int 32
    RGB32I,
    ///
    ///
    /// Stored in 4 uint32
    RGB32UI,
    ///
    ///
    /// Stored in 4 uint4
    RGBA4,
    /// 32 bits RGBA pixel
    ///
    /// Stored in 4 uint8
    RGBA8,
    ///
    ///
    /// Stored in 4 half
    RGBA16F,
    ///
    ///
    /// Stored in 4 int16
    RGBA16I,
    ///
    ///
    /// Stored in 4 uint16
    RGBA16UI,
    ///
    ///
    /// Stored in 4 floats
    RGBA32F,
    ///
    ///
    /// Stored in 4 int32
    RGBA32I,
    ///
    ///
    /// Stored in 4 uint32
    RGBA32UI,
    ///
    ///
    /// Stored in
    Depth16,
    ///
    ///
    /// Stored in
    Depth24,
    ///
    ///
    /// Stored in
    Depth24Stencil8,
    ///
    ///
    /// Stored in
    Depth32,
    ///
    ///
    /// Stored in
    Stencil1,
    ///
    ///
    /// Stored in
    Stencil4,
    ///
    ///
    /// Stored in
    Stencil8,
    ///
    ///
    /// Stored in
    Stencil16,
}

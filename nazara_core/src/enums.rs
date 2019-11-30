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
/// Represents the content of a pixel format
pub enum PixelFormatContent {
    /// Pixel format has at least one of the RGBA component
    ColorRGBA,
    /// Pixel format is used to store depth and stencil informations
    DepthStencil,
    /// Pixel format is used to store stencil informations,
    Stencil,
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
    /// 8 bits red signed integral pixel
    ///
    /// Stored in 1 int8
    R8I,
    /// 8 bits red unsigned integral pixel
    ///
    /// Stored in 1 uint8
    R8UI,
    /// 16 bits red only pixel
    ///
    /// Stored in 1 uint16
    R16,
    /// 16 bits red float pixel
    ///
    /// Stored in 1 half
    R16F,
    /// 16 bits red signed integral pixel
    ///
    /// Stored in 1 int16
    R16I,
    /// 16 bits red unsigned integral pixel
    ///
    /// Stored in 1 uint16
    R16UI,
    /// 32 bits red float pixel
    ///
    /// Stored in 1 float
    R32F,
    /// 32 bits red signed integral pixel
    ///
    /// Stored in 1 uint16
    R32I,
    /// 32 bits red unsigned integral pixel
    ///
    /// Stored in 1 uint32
    R32UI,
    /// 16 bits red/green pixel
    ///
    /// Stored in 2 int8
    RG8,
    /// 8 bits red/green signed integral pixel
    ///
    /// Stored in 2 int8
    RG8I,
    /// 8 bits red/green unsigned integral pixel
    ///
    /// Stored in 2 uint8
    RG8UI,
    /// 32 bits red/green pixel
    ///
    /// Stored in 2 uint16
    RG16,
    /// 16 bits red/green float pixel
    ///
    /// Stored in 2 half
    RG16F,
    /// 16 bits red/green signed integral pixel
    ///
    /// Stored in 2 int16
    RG16I,
    /// 16 bits red/green unsigned integral pixel
    ///
    /// Stored in 2 uint16
    RG16UI,
    /// 32 bits red/green float pixel
    ///
    /// Stored in 2 float
    RG32F,
    /// 32 bits red/green signed integral pixel
    ///
    /// Stored in 2 uint16
    RG32I,
    /// 32 bits red/green unsigned integral pixel
    ///
    /// Stored in 2 uint32
    RG32UI,
    /// 16-bits RGBA pixel
    ///
    /// Stored in 3 uint5 and 1 alpha bit
    RGB5A1,
    /// 24 bits RGB pixel
    ///
    /// Stored in 3 uint8
    RGB8,
    /// 16 bits RGB pixel
    ///
    /// Stored in 3 half
    RGB16F,
    /// 16 bits RGB signed integral pixel
    ///
    /// Stored in 4 int16
    RGB16I,
    /// 16 bits RGB unsigned integral pixel
    ///
    /// Stored in 4 uint16
    RGB16UI,
    /// 32 bits RGB pixel
    ///
    /// Stored in 3 float
    RGB32F,
    /// 32 bits RGB signed integral pixel
    ///
    /// Stored in 4 int 32
    RGB32I,
    /// 32 bits RGB unsigned integral pixel
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
    /// 16 bits RGBA float pixel
    ///
    /// Stored in 4 half
    RGBA16F,
    /// 16 bits RGBA signed integral pixel
    ///
    /// Stored in 4 int16
    RGBA16I,
    /// 16 bits RGBA unsigned integral
    ///
    /// Stored in 4 uint16
    RGBA16UI,
    /// 32 bits RGBA float pixel
    ///
    /// Stored in 4 floats
    RGBA32F,
    /// 32 bits RGBA signed integral pixel
    ///
    /// Stored in 4 int32
    RGBA32I,
    /// 32 bits RGBA unsigned integral pixel
    ///
    /// Stored in 4 uint32
    RGBA32UI,
    /// 16 bits depth floating pixel
    ///
    /// Stored in
    Depth16,
    /// 24 bits depth floating pixel
    ///
    /// Stored in
    Depth24,
    ///
    ///
    /// Stored in
    Depth24Stencil8,
    /// 32 bits depth floating pixel
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
    /// idk what is it
    ///
    /// Stored in
    Palette,
}

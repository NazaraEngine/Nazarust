#[derive(Clone, Copy)]
pub enum ImageType {
    Array1D,
    Array2D,
    Cubemap,
    Single1D,
    Single2D,
    Single3D,
}

#[derive(Clone, Copy)]
pub enum PixelFormatType {
    A8,    // 1*uint8
    BGR8,  // 3*uint8
    BGRA8, // 4*uint8
    DXT1,
    DXT3,
    DXT5,
    L8,       // 1*uint8
    LA8,      // 2*uint8
    R8,       // 1*uint8
    R8I,      // 1*int8
    R8UI,     // 1*uint8
    R16,      // 1*uint16
    R16F,     // 1*half
    R16I,     // 1*int16
    R16UI,    // 1*uint16
    R32F,     // 1*float
    R32I,     // 1*uint16
    R32UI,    // 1*uint32
    RG8,      // 2*int8
    RG8I,     // 2*int8
    RG8UI,    // 2*uint8
    RG16,     // 2*uint16
    RG16F,    // 2*half
    RG16I,    // 2*int16
    RG16UI,   // 2*uint16
    RG32F,    // 2*float
    RG32I,    // 2*uint16
    RG32UI,   // 2*uint32
    RGB5A1,   // 3*uint5 + alpha bit
    RGB8,     // 3*uint8
    RGB16F,   // 3*half
    RGB16I,   // 4*int16
    RGB16UI,  // 4*uint16
    RGB32F,   // 3*float
    RGB32I,   // 4*int32
    RGB32UI,  // 4*uint32
    RGBA4,    // 4*uint4
    RGBA8,    // 4*uint8
    RGBA16F,  // 4*half
    RGBA16I,  // 4*int16
    RGBA16UI, // 4*uint16
    RGBA32F,  // 4*float
    RGBA32I,  // 4*int32
    RGBA32UI, // 4*uint32
    Depth16,
    Depth24,
    Depth24Stencil8,
    Depth32,
    Stencil1,
    Stencil4,
    Stencil8,
    Stencil16,
}

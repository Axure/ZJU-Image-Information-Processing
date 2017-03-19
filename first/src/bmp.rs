#[repr(C, packed)]
pub struct BmpFile {
    pub bitmap_file_header: [u8; 14],
    pub dib_header: [u8; 14],
    pub extra_bit_masks: [u8; 14],
    pub color_table: [u8; 14],
    pub gap_1: [u8; 14],
    pub pixel_array: [u8; 14],
    pub gap_2: [u8; 14],
    pub icc_color_profile: [u8; 14],
}

pub struct DummyFile {
    pub info: [u8; 14]
}

#[repr(C, packed)]
pub struct BitmapFileHeader {
    pub header_field: [u8; 2],
    pub size_in_bytes: [u8; 4],
    pub reserved_1: [u8; 2],
    pub reserved_2: [u8; 2],
    pub offset: [u8; 2],
}

#[repr(C)]
pub enum DIBHeader {
    #[repr(C, packed)]
    BITMAPCOREHEADER {
        header_size: [u8; 4],
        width: [u8; 4],
        height: [u8; 4],
        color_planes: [u8; 2],
        bits_per_pixel: [u8; 2],
    },
    #[repr(C, packed)]
    BITMAPINFOHEADER {
        header_size: [u8; 4],
        width: [u8; 4],
        height: [u8; 4],
        color_planes: [u8; 2],
        bits_per_pixel: [u8; 2],
        compression_method: CompressionMethod,
        image_size: [u8; 4],
        horizontal_resolution: [u8; 4],
        vertical_resolution: [u8; 4],
        colors_in_palette: [u8; 4],
        important_colors_used: [u8; 4],

    }
}

#[repr(u32)]
enum CompressionMethod {
    BI_RGB = 0,
    BI_RLE8 = 1,
    BI_RLE4 = 2,
    BI_BITFIELDS = 3,
    BI_JPEG = 4,
    BI_PNG = 5,
    BI_ALPHABITFIELDS = 6,
}


#[repr(C, packed)]
pub struct BITMAPINFOHEADER {

}

#[repr(C, packed)]
pub struct ExtractBitMasks {}

#[repr(C, packed)]
pub struct ColorTable {}

#[repr(C, packed)]
pub struct PixelArray {}

enum Bitmap_Header_Start {}

enum HEADER {
    GOOD,
    BAD
}


impl BmpFile {
    pub fn new() -> DummyFile {
        return DummyFile {
            info: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        };
    }
}
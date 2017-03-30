use memmap::{Mmap, Protection};

pub struct BmpMmapedFile {
    path: String,
    data: Mmap
}

pub trait BmpHeader {
    fn get_header_field(&self) -> u16;
    fn get_size(&self) -> u32;
    fn get_offset(&self) -> u32;
}

pub trait DIBHeader {
    fn get_dib_header_size(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_color_depth(&self) -> u16;
}

impl BmpMmapedFile {
    pub fn new(path: &str) -> BmpMmapedFile {
        BmpMmapedFile {
            path: String::from(path),
            data: Mmap::open_path(path, Protection::Read).unwrap()
        }
    }

    pub fn get_array(&self) -> &[u8] {
        return unsafe { self.data.as_slice() }
    }

    fn get_dib_array(&self) -> &[u8] {
        let length = match self.get_header_field() {
            0x4D42 => ((self.get_array()[14] as u32) << 8) + self.get_array()[15] as u32,
            _ => 0
        };
        return &self.get_array()[14..14 + length as usize]
    }

    fn get_pixel_array(&self) -> &[u8] {
        let offset = self.get_offset();
        return &self.get_array()[offset as usize
            ..
            (offset as usize + self.get_pixel_array_length())]
    }

    pub fn get_pixel_array_length(&self) -> usize {
        return (self.get_width()
            * self.get_height()
            * (self.get_color_depth() as u32 / 8)) as usize
    }

    pub fn get_pixel_count(&self) -> usize {
        return (self.get_width()
            * self.get_height()) as usize
    }


    pub fn get_r_mask(&self) -> u32 {
        return ((self.get_dib_array()[43] as u32) << 24)
            + ((self.get_dib_array()[42] as u32) << 16)
            + ((self.get_dib_array()[41] as u32) << 8)
            + self.get_dib_array()[40] as u32
    }


    pub fn get_g_mask(&self) -> u32 {
        return ((self.get_dib_array()[47] as u32) << 24)
            + ((self.get_dib_array()[46] as u32) << 16)
            + ((self.get_dib_array()[45] as u32) << 8)
            + self.get_dib_array()[44] as u32
    }


    pub fn get_b_mask(&self) -> u32 {
        return ((self.get_dib_array()[51] as u32) << 24)
            + ((self.get_dib_array()[50] as u32) << 16)
            + ((self.get_dib_array()[49] as u32) << 8)
            + self.get_dib_array()[48] as u32
    }


    pub fn get_a_mask(&self) -> u32 {
        return ((self.get_dib_array()[55] as u32) << 24)
            + ((self.get_dib_array()[54] as u32) << 16)
            + ((self.get_dib_array()[53] as u32) << 8)
            + self.get_dib_array()[52] as u32
    }
}

pub trait CloneWithTransformation {
    fn clone_with() {}
}

impl BmpHeader for BmpMmapedFile {
    fn get_size(&self) -> u32 {
        //        unsafe { std::mem::transmute::<&[u8], u32>(self.get_array()) }
        return ((self.get_array()[5] as u32) << 24) // TODO: rewrite using some `transmute` thing?
            + ((self.get_array()[4] as u32) << 16)
            + ((self.get_array()[3] as u32) << 8)
            + self.get_array()[2] as u32
    }
    fn get_header_field(&self) -> u16 {
        return ((self.get_array()[1] as u16) << 8)
            + self.get_array()[0] as u16
    }

    fn get_offset(&self) -> u32 {
        return ((self.get_array()[13] as u32) << 24)
            + ((self.get_array()[12] as u32) << 16)
            + ((self.get_array()[11] as u32) << 8)
            + self.get_array()[10] as u32
    }
}

impl DIBHeader for BmpMmapedFile {
    fn get_dib_header_size(&self) -> u32 {
        return ((self.get_dib_array()[3] as u32) << 24)
            + ((self.get_dib_array()[2] as u32) << 16)
            + ((self.get_dib_array()[1] as u32) << 8)
            + self.get_dib_array()[0] as u32
    }

    fn get_width(&self) -> u32 {
        return ((self.get_dib_array()[7] as u32) << 24)
            + ((self.get_dib_array()[6] as u32) << 16)
            + ((self.get_dib_array()[5] as u32) << 8)
            + self.get_dib_array()[4] as u32
    }

    fn get_height(&self) -> u32 {
        return ((self.get_dib_array()[11] as u32) << 24)
            + ((self.get_dib_array()[10] as u32) << 16)
            + ((self.get_dib_array()[9] as u32) << 8)
            + self.get_dib_array()[8] as u32
    }

    fn get_color_depth(&self) -> u16 {
        return ((self.get_dib_array()[15] as u16) << 8)
            + self.get_dib_array()[14] as u16
    }
}
extern crate memmap;

mod bmp;

use bmp::BmpFile;


use memmap::{Mmap, Protection};

struct BmpMmapedFile {
    path: String,
    data: Mmap
}

trait BmpHeader {
    fn get_header_field(&self) -> u16;
    fn get_size(&self) -> u32;
    fn get_offset(&self) -> u32;
}

trait DIBHeader {
    fn get_dib_header_size(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_color_depth(&self) -> u16;
}

impl BmpMmapedFile {
    fn new(path: &str) -> BmpMmapedFile {
        BmpMmapedFile {
            path: String::from(path),
            data: unsafe { (Mmap::open_path(path, Protection::Read).unwrap()) }
        }
    }

    fn get_array(&self) -> &[u8] {
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

    fn get_pixel_array_length(&self) -> usize {
        return (self.get_width()
            * self.get_height()
            * (self.get_color_depth() as u32 / 8)) as usize
    }

    fn get_pixel_count(&self) -> usize {
        return (self.get_width()
            * self.get_height()) as usize
    }


    fn get_r_mask(&self) -> u32 {
        return ((self.get_dib_array()[43] as u32) << 24)
            + ((self.get_dib_array()[42] as u32) << 16)
            + ((self.get_dib_array()[41] as u32) << 8)
            + self.get_dib_array()[40] as u32
    }


    fn get_g_mask(&self) -> u32 {
        return ((self.get_dib_array()[47] as u32) << 24)
            + ((self.get_dib_array()[46] as u32) << 16)
            + ((self.get_dib_array()[45] as u32) << 8)
            + self.get_dib_array()[44] as u32
    }


    fn get_b_mask(&self) -> u32 {
        return ((self.get_dib_array()[51] as u32) << 24)
            + ((self.get_dib_array()[50] as u32) << 16)
            + ((self.get_dib_array()[49] as u32) << 8)
            + self.get_dib_array()[48] as u32
    }


    fn get_a_mask(&self) -> u32 {
        return ((self.get_dib_array()[55] as u32) << 24)
            + ((self.get_dib_array()[54] as u32) << 16)
            + ((self.get_dib_array()[53] as u32) << 8)
            + self.get_dib_array()[52] as u32
    }
}

trait CloneWithTransformation {
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

fn transform_color(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let mut y = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    let u = -0.14713 * r as f64 + (-0.28886) * g as f64 + 0.436 * b as f64;
    let v = 0.615 * r as f64 + (-0.51499) * g as f64 + (-0.10001) * b as f64;

    //    println!("y is {}", y);
    y = 0.0;
    //    println!("y is {}", y);

    let r_new = (1.0 * y + 0.0 * u + 1.13983 * v) as u8;
    let g_new = (1.0 * y + (-0.39465) * u + -0.5806 * v) as u8;
    let b_new = (1.0 * y + (2.03211) * u + 0.0 * v) as u8;
    return (r_new, g_new, b_new)
}

fn main() {
    use std::io::Write;

    let original_bmp = BmpMmapedFile::new("resources/colorful.bmp");
    let original_bytes = original_bmp.get_array();

    println!("The header field of the bmp file is {:X}.", original_bmp.get_header_field());
    println!("The size of the bmp file is {}.", original_bmp.get_size());
    println!("The offset of the bmp file is {}.", original_bmp.get_offset());

    println!("The size of dib header of the bmp file is {}.", original_bmp.get_dib_header_size());
    println!("The width of the bmp file is {}.", original_bmp.get_width());
    println!("The height of the bmp file is {}.", original_bmp.get_height());
    println!("The color depth of the bmp file is {}.", original_bmp.get_color_depth());

    println!("The r mask is {:032b}.", original_bmp.get_r_mask());
    println!("The g mask is {:032b}.", original_bmp.get_g_mask());
    println!("The b mask is {:032b}.", original_bmp.get_b_mask());
    println!("The a mask is {:032b}.", original_bmp.get_a_mask());

    println!("{:?}", &original_bytes[0..8]);
    println!("{:?}", &original_bytes[10..14]);
    println!("{:?}", &original_bytes[138..142]);
    println!("{:?}", &original_bytes[142..146]);
    println!("{:?}", original_bytes.len());

    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("resources/foo.bmp").unwrap();
    unsafe { file.set_len(original_bytes.len() as u64); }
    file.sync_all();

    let mut duplicated_mmap = Mmap::open_path("resources/foo.bmp", Protection::ReadWrite).unwrap();
    {
        let duplicated_bytes: &[u8] = unsafe { duplicated_mmap.as_slice() };
        println!("{:?}", duplicated_bytes.len());
        println!("{:?}", &duplicated_bytes[0..8]);
    }
    unsafe { duplicated_mmap.as_mut_slice() }.write(original_bytes).unwrap();
    {
        let duplicated_bytes: &mut [u8] = unsafe { duplicated_mmap.as_mut_slice() };
        println!("The length of the pixels is {}", original_bmp.get_pixel_array_length());
        let initial_offset = original_bmp.get_offset() as usize;
        for i in 0..original_bmp.get_pixel_count() {
            let extra_offset = i * 4;
            let (r, g, b) = transform_color(
                duplicated_bytes[initial_offset + extra_offset + 3],
                duplicated_bytes[initial_offset + extra_offset + 2],
                duplicated_bytes[initial_offset + extra_offset + 1]
            );
            duplicated_bytes[initial_offset + extra_offset + 3] = r;
            duplicated_bytes[initial_offset + extra_offset + 2] = g;
            duplicated_bytes[initial_offset + extra_offset + 1] = b;
        }

        println!("{:?}", duplicated_bytes.len());
        println!("{:?}", &duplicated_bytes[0..8]);
    }
    duplicated_mmap.flush();
}

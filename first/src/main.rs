extern crate memmap;

mod bmp;

use bmp::BmpMmapedFile;
use bmp::BmpHeader;
use bmp::DIBHeader;


use memmap::{Mmap, Protection};


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

#[warn(unused_must_use)]
fn main() {
    use std::io::Write;

    let original_bmp = BmpMmapedFile::new("resources/colorful.bmp");

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


    use std::fs::File;

    let file = File::create("resources/foo.bmp").unwrap();
    unsafe { file.set_len(original_bmp.get_array().len() as u64); }
    file.sync_all();

    let mut duplicated_mmap = Mmap::open_path("resources/foo.bmp", Protection::ReadWrite).unwrap();
    {
        let duplicated_bytes: &[u8] = unsafe { duplicated_mmap.as_slice() };
        println!("{:?}", duplicated_bytes.len());
        println!("{:?}", &duplicated_bytes[0..8]);
    }
    unsafe { duplicated_mmap.as_mut_slice() }.write(original_bmp.get_array()).unwrap();
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

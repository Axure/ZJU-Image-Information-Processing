extern crate memmap;

mod bmp;

use bmp::BmpFile;

fn main() {
    use std::io::Write;
    use memmap::{Mmap, Protection};

    let original_mmap = Mmap::open_path("resources/colorful.bmp", Protection::Read).unwrap();
    let original_bytes: &[u8] = unsafe { original_mmap.as_slice() };
    println!("{:?}", &original_bytes[0..8]);
    println!("{:?}", &original_bytes[10..14]);
    println!("{:?}", &original_bytes[138..142]);
    println!("{:?}", &original_bytes[142..146]);
    println!("{:?}", original_bytes.len());

    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("resources/foo.bmp").unwrap();
    unsafe {file.set_len(original_bytes.len() as u64);}
    file.sync_all();

    let mut duplicated_mmap = Mmap::open_path("resources/foo.bmp", Protection::ReadWrite).unwrap();
    {
        let duplicated_bytes: &[u8] = unsafe { duplicated_mmap.as_slice() };
        println!("{:?}", duplicated_bytes.len());
        println!("{:?}", &duplicated_bytes[0..8]);
    }
    unsafe { duplicated_mmap.as_mut_slice() }.write(original_bytes).unwrap();
    {
        let anon_bytes: &[u8] = unsafe { duplicated_mmap.as_slice() };
        println!("{:?}", anon_bytes.len());
        println!("{:?}", &anon_bytes[0..8]);
    }
    duplicated_mmap.flush();

}

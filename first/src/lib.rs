use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::error::Error;

#[test]
fn test() {
    /// Open the file
    let path = Path::new("resources/wow.bmp");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open file because {}", why),
        Ok(file) => file,
    };

    let s = String::new();
    let take = file.take(240u64);
    let mut bytes = take.bytes();
//    println!("The count is {}", bytes.count()); // TODO: understand why it would cause a `move` error.

    loop {
        match bytes.next() {
            Some(x) => {
                match x{
                    Err(why) => panic!(why),
                    Ok(x) => print!("{:X} ", x)
                }
            }
            None => { break }
        }
    }
}
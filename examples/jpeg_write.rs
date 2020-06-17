//! An example of converting an image to JPEG.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{ImageOutputFormat, GenericImageView};

fn main() {
    let file = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let output = if env::args().count() == 3 {
        env::args().nth(2).unwrap()
    } else {
        file.clone()
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let file_stem = Path::new(&file).file_stem();
    let out_path = format!("{}{}.jpg", output, file_stem.unwrap().to_str().unwrap());
    println!("{}", out_path);

    let fout = &mut File::create(&Path::new(&out_path)).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    im.write_to(fout, ImageOutputFormat::Jpeg(75)).unwrap();
}

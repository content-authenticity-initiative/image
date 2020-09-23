//! An example of converting an image to JPEG.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{ImageOutputFormat, GenericImageView, ColorType, jpeg::JpegEncoder};

fn main() {
    // compute the source image and output directory
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
    let (_width, _height) = im.dimensions();
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    // create a new output path based on the input name + output path
    let file_stem = Path::new(&file).file_stem();
    let out_path = format!("{}{}.jpg", output, file_stem.unwrap().to_str().unwrap());
    println!("{}", out_path);

    // create the output file
    let fout = &mut File::create(&Path::new(&out_path)).unwrap();

    // get a simple RGB image out
    // then create the JPEG encoder, with our settings
    // and have it write it out!
    let img_bits = im.into_rgb();
    let mut encoder = JpegEncoder::new_with_quality(fout, 75);
    encoder.encode_image(&img_bits).expect("Could not encode image");
}

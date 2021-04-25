extern crate image;

use image::io::Reader as ImageReader;
use image::ImageFormat;
use image::{Rgba};

use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

use std::env;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    println!("Loading file...");
    // let mut im = image::open(&Path::new(&file)).unwrap();
    let bytes = get_bytes_from_file(&file);
    let height = get_dims(bytes);
    println!("Height is {}", height);
}

fn get_dims(byteData: Vec<u8>) -> u32 {
    let reader = ImageReader::new(Cursor::new(byteData))
    .with_guessed_format()
    .expect("Cursor io never fails");

    let img = reader.decode().unwrap().to_rgba8();
    img.height()
}

fn get_height_from_bytes(byteData: Vec<u8>) -> u32 {
    let newImage = image::load_from_memory(&byteData).unwrap().to_rgba8();
    return newImage.height();
}

fn write_file(im: image::DynamicImage, file: &str) {
    let fout = &mut File::create(&Path::new(&format!("{}.jpeg", file))).unwrap();
    im.write_to(fout, ImageFormat::Jpeg).unwrap();
}

fn add_text_to_image(mut im: image::DynamicImage) -> image::DynamicImage {
    let font =
        Vec::from(include_bytes!("/home/aadityavs/Projects/image-ed/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 120.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    let color = Rgba([0u8, 0u8, 255u8, 255u8]);

    let text = "Hello, world!";
    draw_text_mut(&mut im, color, 0, 0, scale, &font, text);
    return im;
}

fn get_bytes_from_file(file: &str) -> Vec<u8> {
    let im = image::open(&Path::new(&file)).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    im.write_to(&mut bytes, image::ImageOutputFormat::Png)
        .unwrap();
    return bytes;
}

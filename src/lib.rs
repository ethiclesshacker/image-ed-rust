extern crate image;

use image::DynamicImage;
// use image::{RgbImage, Rgba};
use image::io::Reader as ImageReader;


use std::format;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[wasm_bindgen]
pub fn get_dimensions_from_bytes(byteData: Vec<u8>) -> Vec<u8> {
    web_sys::console::log_1(&"Hello, world!".into());
    // web_sys::console::log(byteData);
    // let newImage = image::load_from_memory(&byteData).unwrap();
    
    return byteData;

    // let reader = ImageReader::new(Cursor::new(byteData))
        // .with_guessed_format()
        // .expect("Cursor io never fails");
    // let img = reader.decode().unwrap();
    // let height = img.height();
    // web_sys::console::log_1(&format!("{}", height).into());

    // return newImage.height();
}

#[wasm_bindgen]
pub fn get_dims(byteData: Vec<u8>) -> u32 {
    web_sys::console::log_1(&"Hello, world!".into());
    let reader = ImageReader::new(Cursor::new(byteData))
        .with_guessed_format()
        .expect("Cursor io never fails");
    let fmt = reader.format();
    web_sys::console::log_1(&format!("Guessing done. {:?}", fmt).into());
    // let img = reader.decode().unwrap().to_rgba8();
    let height = reader.into_dimensions().unwrap().1;
    // let height = img.height();
    web_sys::console::log_1(&format!("{}", height).into());
    return height;
}

#[wasm_bindgen]
pub fn blur(byteData: Vec<u8>, sigma: f32) -> Vec<u8> {
    let temp_image = get_image_from_bytes(byteData);
    return get_bytes_from_image(&DynamicImage::blur(&temp_image, sigma));
}

fn get_bytes_from_image(im: &DynamicImage) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    im.write_to(&mut bytes, image::ImageOutputFormat::Png)
        .unwrap();
    return bytes;
}

fn get_image_from_bytes(byteData: Vec<u8>) -> DynamicImage {
    let newImage = image::load_from_memory(&byteData).unwrap();
    return newImage;
}

/*
fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    println!("Loading file...");
    let mut im = image::open(&Path::new(&file)).unwrap();

    im = add_text_to_image(im);
    write_file(im, "text-added");
}

fn write_file(im: DynamicImage, file: &str) {
    let fout = &mut File::create(&Path::new(&format!("{}.jpeg", file))).unwrap();
    im.write_to(fout, ImageFormat::Jpeg).unwrap();
}

fn add_text_to_image(mut im: DynamicImage) -> DynamicImage {
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

fn get_dimensions_from_bytes(byteData: Vec<u8>) -> (u32, u32) {
    let newImage = image::load_from_memory(&byteData).unwrap().to_rgb8();
    return newImage.dimensions();
}

*/

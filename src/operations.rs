fn blurring_and_save(im: image::DynamicImage) {
    println!("Blurring file...");
    let file = "blurred-image";
    image::imageops::blur(&im, 5.0)
        .save(&Path::new(&format!("{}.png", file)))
        .unwrap();
}

fn try_image() -> Result<(), ImageError> {
    let img = ImageReader::open("pexels-pixabay-210158.jpg")?.decode()?;
    Ok(())
}

fn get_bytes_from_image(im: &image::DynamicImage) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    im.write_to(&mut bytes, image::ImageOutputFormat::Png)
        .unwrap();
    return bytes;
}

fn get_bytes_from_file(file: &str) -> Vec<u8> {
    let im = image::open(&Path::new(&file)).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    im.write_to(&mut bytes, image::ImageOutputFormat::Png)
        .unwrap();
    return bytes;
}

fn get_image_from_bytes(byteData: Vec<u8>) -> image::DynamicImage {
    let newImage = image::load_from_memory(&byteData).unwrap();
    return newImage;
}

fn get_dimensions_from_bytes(byteData: Vec<u8>) -> image::DynamicImage {
    let newImage = image::load_from_memory(&byteData).unwrap();
    return newImage.dimensions();
}

use dither::DitherImage;
use image::{DynamicImage, ImageReader};
mod dither;
fn main() {
    let filename = "input.jpg";

    let image_file = ImageReader::open(filename).unwrap();
    let image_file = image_file.decode().unwrap();
    let dither_image = DitherImage::new(image_file)
        .highlights((255, 255, 255))
        .shadows((0, 0, 0))
        .generate();
    dither_image.image.save("output.jpg").unwrap();
}

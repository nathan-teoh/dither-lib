use dither::{DitherBuilder, Resize};
use image::ImageReader;
mod dither;
fn main() {
    let filename = "input.jpg";

    let image_file = ImageReader::open(filename).unwrap();
    let image_file = image_file.decode().unwrap();
    let dither_image = DitherBuilder::new(image_file)
        .highlights((255, 255, 255))
        .shadows((0, 0, 0))
        // .resize(Resize::Scale(0.5))
        .generate();
    dither_image.save("output2.jpg").unwrap();
}

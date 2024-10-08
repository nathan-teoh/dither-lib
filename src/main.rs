use dither::{DitherBuilder, Resize};
use image::ImageReader;
mod dither;
// Use the re-exported `Rgb`
use dither::Rgb;
fn main() {
    let filename = "input.jpg";

    let image_file = ImageReader::open(filename).unwrap();
    let image_file = image_file.decode().unwrap();
    let dither_image = DitherBuilder::new(image_file)
        .highlights(Rgb([255; 3]))
        .shadows(Rgb([0; 3]))
        .resize(Resize::Scale(0.5))
        .generate();
    dither_image.save("output.jpg").unwrap();
}

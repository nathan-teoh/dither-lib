use dither::DitherImage;
use image::DynamicImage;
mod dither;
fn main() {
    let dyn_img: DynamicImage = Default::default();
    let dither_image = DitherImage::new(dyn_img)
        .width(200)
        .height(400)
        .highlights((255, 255, 255))
        .shadows((0, 0, 0))
        .generate();
}

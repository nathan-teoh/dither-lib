//! This is a simple dithering library
//!
//! # Usage
//! ```rust
//!# use image::DynamicImage;
//!# use dither_lib::{DitherBuilder,Rgb,Resize};
//!# pub fn main(){
//!     // Obviously this will not work...
//!     let image: DynamicImage = DynamicImage::default();
//!     let dithered_image = DitherBuilder::new(image)
//!         .highlights(Rgb([255;3]))
//!         .shadows(Rgb([0;3]))
//!         .resize(Resize::Scale(0.5))
//!         .generate();
//!# }
//!```
//re-export `image`'s `Rgb<_>` struct
pub use image::Rgb;
use image::{imageops::FilterType, DynamicImage};
use ndarray::{arr2, concatenate, Array2, Axis};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

const R_CHANNEL_MULTIPLIER: f64 = 0.2126;
const G_CHANNEL_MULTIPLIER: f64 = 0.7152;
const B_CHANNEL_MULTIPLIER: f64 = 0.0722;

/// Dithered image builder
pub struct DitherBuilder {
    image: DynamicImage,
    level: u8,
    width: u32,
    height: u32,
    shadows: Rgb<u8>,
    highlights: Rgb<u8>,
}
impl DitherBuilder {
    /// Initializes a new `DitherBuilder`
    pub fn new(image: DynamicImage) -> DitherBuilder {
        let width = image.width();
        let height = image.height();
        DitherBuilder {
            image,
            width,
            height,
            shadows: Rgb([0; 3]),
            highlights: Rgb([255; 3]),
            level: 2,
        }
    }
}

pub enum Resize {
    Scale(f32),
    Resolution { width: u32, height: u32 },
}

impl DitherBuilder {
    /// Sets the dithering level
    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }
    /// Resizes the output image
    pub fn resize(mut self, resize: Resize) -> Self {
        match resize {
            Resize::Scale(scale) => {
                self.width = (scale * self.width as f32) as u32;
                self.height = (scale * self.height as f32) as u32;
            }
            Resize::Resolution { width, height } => {
                self.width = width;
                self.height = height;
            }
        };
        self
    }

    /// Sets the color of highlights in the dithered image
    pub fn highlights(mut self, highlights: Rgb<u8>) -> Self {
        self.highlights = highlights;
        self
    }
    /// Sets the color of the shadows in the dithered image
    pub fn shadows(mut self, shadows: Rgb<u8>) -> Self {
        self.shadows = shadows;
        self
    }
    /// Generate a dithered image given a set of parameters and returns a DynamicImage
    pub fn generate(self) -> DynamicImage {
        //generate equalizer
        let num = 2_u8.pow(self.level.into());
        let equalizer = 1. / (num as f32).powf(2.);
        //generate bayer layer
        let bayer_layer = generate_bayer(self.level).mapv(|x| (x as f32) * equalizer);
        //convert to grayscale
        let image = self.image.grayscale();
        //resize image
        let image = image.resize(self.width, self.height, FilterType::Nearest);
        let mut binding = image.to_rgb8();
        let mut image_buffer: Vec<_> = binding.enumerate_pixels_mut().collect();
        image_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(_, (x, y, pixel))| {
                let bayer_len = bayer_layer.shape()[0];
                let x = x.rem_euclid(bayer_len.try_into().unwrap()) as usize;
                let y = y.rem_euclid(bayer_len.try_into().unwrap()) as usize;
                let pixel_brightness = pixel_brightness(pixel);
                if pixel_brightness > (1. - bayer_layer[[y, x]]).into() {
                    set_pixel(*pixel, self.highlights);
                } else {
                    set_pixel(*pixel, self.shadows);
                }
            });

        let mut new_img_buffer = DynamicImage::new_rgb8(self.width, self.height).to_rgb8();
        for (x, y, pixel) in image_buffer {
            new_img_buffer.put_pixel(x, y, *pixel);
        }
        DynamicImage::ImageRgb8(new_img_buffer)
    }
}

fn pixel_brightness(pixel: &Rgb<u8>) -> f64 {
    let r = pixel[0] as f64 / 255.;
    let g = pixel[1] as f64 / 255.;
    let b = pixel[2] as f64 / 255.;

    let pixel_brightness =
        r * R_CHANNEL_MULTIPLIER + g * G_CHANNEL_MULTIPLIER + b * B_CHANNEL_MULTIPLIER;
    gamma_correct(pixel_brightness)
}

fn set_pixel(pixel: &mut Rgb<u8>, color: Rgb<u8>) {
    pixel[0] = color[0];
    pixel[1] = color[1];
    pixel[2] = color[2];
}

fn generate_bayer(level: u8) -> Array2<i32> {
    let num = 2_u8.pow(level.into());
    match num {
        2 => arr2(&[[0, 2], [3, 1]]),
        _ => {
            concatenate![
                Axis(0),
                concatenate![
                    Axis(1),
                    4 * generate_bayer(level - 1),
                    4 * generate_bayer(level - 1) + 2
                ],
                concatenate![
                    Axis(1),
                    4 * generate_bayer(level - 1) + 3,
                    4 * generate_bayer(level - 1) + 1
                ]
            ]
        }
    }
}

fn gamma_correct(pixel_brightness: f64) -> f64 {
    if pixel_brightness <= 0.0405 {
        pixel_brightness / 12.92
    } else {
        (pixel_brightness + 0.055 / 1.055).powf(2.4)
    }
}

use image::{imageops::FilterType, DynamicImage};
use ndarray::{arr2, concatenate, Array2, Axis};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct DitherImage {
    image: DynamicImage,
}

pub struct DitherBuilder {
    image: DynamicImage,
    level: u8,
    width: u32,
    height: u32,
    shadows: (u8, u8, u8),
    highlights: (u8, u8, u8),
}
impl DitherImage {
    pub fn new(image: DynamicImage) -> DitherBuilder {
        DitherBuilder {
            image: image,
            width: 0,
            height: 0,
            shadows: (0, 0, 0),
            highlights: (255, 255, 255),
            level: 2,
        }
    }
}

impl DitherBuilder {
    pub fn level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    // Sets the width of the output image
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    // Sets the height of the output image
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    // Sets the color of highlights in the dithered image
    pub fn highlights(mut self, highlights: (u8, u8, u8)) -> Self {
        self.highlights = highlights;
        self
    }
    // Sets the color of the shadows in the dithered image
    pub fn shadows(mut self, shadows: (u8, u8, u8)) -> Self {
        self.shadows = shadows;
        self
    }
    // Generate a dithered image given a set of parameters
    pub fn generate(self) -> DitherImage {
        //generate equalizer
        let num = 2_u8.pow(self.level.into());
        let equalizer = 1. / (num as f32).powf(2.);
        //generate bayer layer
        let bayer_layer = generate_bayer(self.level);
        let bayer_layer = bayer_layer.mapv(|x| (x as f32) * equalizer);
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
                let r = pixel[0] as f64 / 255.;
                let g = pixel[1] as f64 / 255.;
                let b = pixel[2] as f64 / 255.;

                let mut br = r * 0.2126 + g * 0.7152 + b * 0.0722;
                let bayer_len = bayer_layer.shape()[0];
                let x = x.rem_euclid(bayer_len.try_into().unwrap()) as usize;
                let y = y.rem_euclid(bayer_len.try_into().unwrap()) as usize;
                br = {
                    if br <= 0.0405 {
                        br / 12.92
                    } else {
                        ((br + 0.055 / 1.055).powf(2.4))
                    }
                };
                if br > (1. - bayer_layer[[y, x]]).into() {
                    pixel[0] = self.highlights.0;
                    pixel[1] = self.highlights.1;
                    pixel[2] = self.highlights.2;
                } else {
                    pixel[0] = self.shadows.0;
                    pixel[1] = self.shadows.1;
                    pixel[2] = self.shadows.2;
                }
            });

        let mut new_img_buffer = DynamicImage::new_rgb8(self.width, self.height).to_rgb8();
        for (x, y, pixel) in image_buffer {
            new_img_buffer.put_pixel(x, y, *pixel);
        }
        DitherImage {
            image: DynamicImage::ImageRgb8(new_img_buffer),
        }
    }
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

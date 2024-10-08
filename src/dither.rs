use image::DynamicImage;

pub struct DitherImage {}

pub struct DitherBuilder {
    width: i32,
    height: i32,
    shadows: (i32, i32, i32),
    highlights: (i32, i32, i32),
}
impl DitherImage {
    pub fn new(image: DynamicImage) -> DitherBuilder {
        DitherBuilder {
            width: 0,
            height: 0,
            shadows: (0, 0, 0),
            highlights: (255, 255, 255),
        }
    }
}

impl DitherBuilder {
    // Sets the width of the output image
    pub fn width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }
    // Sets the height of the output image
    pub fn height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }
    // Sets the color of highlights in the dithered image
    pub fn highlights(mut self, highlights: (i32, i32, i32)) -> Self {
        self.highlights = highlights;
        self
    }
    // Sets the color of the shadows in the dithered image
    pub fn shadows(mut self, shadows: (i32, i32, i32)) -> Self {
        self.shadows = shadows;
        self
    }
    // Generate a dithered image given a set of parameters
    pub fn generate(self) -> DitherImage {
        DitherImage {}
    }
}

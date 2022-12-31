use {
    crate::Color,
    image::{DynamicImage, GrayAlphaImage, GrayImage, RgbImage, RgbaImage},
};

pub enum Image {
    Gray(GrayImage),
    GrayAlpha(GrayAlphaImage),
    Rgb(RgbImage),
    Rgba(RgbaImage),
}

impl Image {
    /// Creates a color from a dynamic image.
    ///
    /// # Errors
    /// Returns an [`Error::UnsupportedFormat`] if given image format is not supported.
    pub fn from_dynamic(im: DynamicImage) -> Result<Self, Error> {
        match im {
            DynamicImage::ImageLuma8(im) => Ok(im.into()),
            DynamicImage::ImageLumaA8(im) => Ok(im.into()),
            DynamicImage::ImageRgb8(im) => Ok(im.into()),
            DynamicImage::ImageRgba8(im) => Ok(im.into()),
            _ => Err(Error::UnsupportedFormat),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match self {
            Self::Gray(im) => (im.width(), im.height()),
            Self::GrayAlpha(im) => (im.width(), im.height()),
            Self::Rgb(im) => (im.width(), im.height()),
            Self::Rgba(im) => (im.width(), im.height()),
        }
    }

    pub fn color(&self, (x, y): (u32, u32)) -> Option<Color> {
        use image::{Pixel, Rgba};

        let rgba = match self {
            Self::Gray(im) => im.get_pixel_checked(x, y).map(Pixel::to_rgba),
            Self::GrayAlpha(im) => im.get_pixel_checked(x, y).map(Pixel::to_rgba),
            Self::Rgb(im) => im.get_pixel_checked(x, y).map(Pixel::to_rgba),
            Self::Rgba(im) => im.get_pixel_checked(x, y).map(Pixel::to_rgba),
        };

        rgba.map(|Rgba(color)| Color::from_byte_array(color))
    }
}

impl From<GrayImage> for Image {
    fn from(v: GrayImage) -> Self {
        Self::Gray(v)
    }
}

impl From<GrayAlphaImage> for Image {
    fn from(v: GrayAlphaImage) -> Self {
        Self::GrayAlpha(v)
    }
}

impl From<RgbImage> for Image {
    fn from(v: RgbImage) -> Self {
        Self::Rgb(v)
    }
}

impl From<RgbaImage> for Image {
    fn from(v: RgbaImage) -> Self {
        Self::Rgba(v)
    }
}

impl TryFrom<DynamicImage> for Image {
    type Error = Error;

    fn try_from(im: DynamicImage) -> Result<Self, Self::Error> {
        Self::from_dynamic(im)
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedFormat,
}

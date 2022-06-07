use image::{DynamicImage, RgbImage};
use opencv::core::{Mat, MatTraitConst, CV_8UC3};

/// Crate error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Input opencv::Mat has invalid dimensions
    #[error("invalid dimensions")]
    InvalidDimensions,
    /// Opencv's crate error
    #[error("opencv error: {0}")]
    Cv(#[from] opencv::Error),
    /// Unsupported underlying format for opencv::Mat
    #[error("unsupported format")]
    UnsupportedFormat,
}

macro_rules! bail {
    ($error:expr) => {
        return Err($error) 
    };
}

/// Represents anything that can be converted into DynamicImage
pub trait ToImage {
    /// Error in conversion
    type Err;

    /// Converts T to DynamicImage
    fn to_image(&self) -> Result<DynamicImage, Self::Err>;
}

impl ToImage for Mat {
    type Err = Error;

    fn to_image(&self) -> Result<DynamicImage, Error> {
        if self.typ() != CV_8UC3 {
            bail!(Error::UnsupportedFormat)
        }
        let w = self.cols();
        if w <= 0 {
            bail!(Error::InvalidDimensions)
        }
        let h = self.rows();
        if h <= 0 {
            bail!(Error::InvalidDimensions)
        }
        let mut rgbim = RgbImage::new(w as u32, h as u32);
        for (pos, pix) in self.iter::<rgb::alt::BGR8>()? {
            let impix = image::Rgb([pix.r, pix.g, pix.b]);
            rgbim.put_pixel(pos.x as u32, pos.y as u32, impix);
        }
        let im = DynamicImage::ImageRgb8(rgbim);
        Ok(im)
    }
}
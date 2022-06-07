use image::{DynamicImage, RgbImage};
use opencv::core::{Mat, MatTraitConst, CV_8UC3};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid dimensions")]
    InvalidDimensions,
    #[error("opencv error: {0}")]
    Cv(#[from] opencv::Error),
    #[error("unsupported format")]
    UnsupportedFormat,
}

macro_rules! bail {
    ($error:expr) => {
        return Err($error) 
    };
}

pub trait ToImage {
    fn to_image(&self) -> Result<DynamicImage, Error>;
}

impl ToImage for Mat {
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
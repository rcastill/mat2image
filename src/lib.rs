use core::slice;

use image::{DynamicImage, RgbImage};
use opencv::{
    core::{Mat, MatTraitConst, CV_8UC3},
    prelude::MatTraitConstManual,
};

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

fn new_rgb_image(mat: &Mat) -> Result<RgbImage, Error> {
    if mat.typ() != CV_8UC3 {
        bail!(Error::UnsupportedFormat)
    }
    let w = mat.cols();
    if w <= 0 {
        bail!(Error::InvalidDimensions)
    }
    let h = mat.rows();
    if h <= 0 {
        bail!(Error::InvalidDimensions)
    }
    Ok(RgbImage::new(w as u32, h as u32))
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
        let mut rgbim = new_rgb_image(self)?;
        for (pos, pix) in self.iter::<rgb::alt::BGR8>()? {
            let impix = image::Rgb([pix.r, pix.g, pix.b]);
            rgbim.put_pixel(pos.x as u32, pos.y as u32, impix);
        }
        let im = DynamicImage::ImageRgb8(rgbim);
        Ok(im)
    }
}

/// Same as `ToImage` but exposing unsafe interface. This is in case source type
/// has an interface to access underlying pointer type, as is the case with
/// `opencv::Mat`
pub trait ToImageUnsafe {
    /// Error in conversion
    type Err;

    /// Converts to DynamicImage (unsafely)
    unsafe fn to_image_unsafe(&self) -> Result<DynamicImage, Self::Err>;
}

impl ToImageUnsafe for Mat {
    type Err = Error;

    unsafe fn to_image_unsafe(&self) -> Result<DynamicImage, Self::Err> {
        let mut rgbim = new_rgb_image(self)?;
        let w = rgbim.width();
        // pixels * 3 channels: already considered in rgbim.len() since it
        // derefs to [P::Subpixel], which is the primitive. See:
        // https://docs.rs/image/0.24.2/image/struct.ImageBuffer.html#deref-methods-%5BP%3A%3ASubpixel%5D
        let data = slice::from_raw_parts(self.data(), rgbim.len());
        for (pixi, i) in (0..data.len()).step_by(3).enumerate() {
            let b = data[i];
            let g = data[i + 1];
            let r = data[i + 2];
            let impix = image::Rgb([r, g, b]);
            let x = pixi as u32 % w;
            let y = pixi as u32 / w;
            rgbim.put_pixel(x, y, impix);
        }
        let im = DynamicImage::ImageRgb8(rgbim);
        Ok(im)
    }
}

#[cfg(test)]
mod test {
    use opencv::imgcodecs::{imread, IMREAD_COLOR};

    use super::*;

    #[test]
    fn safe_and_unsafe_eq() {
        let mat = imread("examples/tinta_helada.jpg", IMREAD_COLOR)
            .expect("Failed to imread");
        let im1 = mat.to_image().expect("Failed to safely convert");
        let im2 = unsafe {
            mat.to_image_unsafe().expect("Failed to UN-safely convert")
        };
        assert_eq!(im1, im2)
    }
}

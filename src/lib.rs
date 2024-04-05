//! Utilities to convert [Mat](https://docs.rs/opencv/latest/opencv/core/struct.Mat.html) to
//! [DynamicImage](https://docs.rs/image/latest/image/enum.DynamicImage.html)

use image::{DynamicImage, RgbImage};
use opencv::{
    core::{MatTraitConst, CV_8UC3},
    prelude::MatTraitConstManual,
};

#[cfg(feature = "experimental")]
mod custom_pix;

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
    #[error("container not big enough: https://docs.rs/image/latest/image/struct.ImageBuffer.html#method.from_raw")]
    ContainerNotBigEnough,
}

macro_rules! bail {
    ($error:expr) => {
        return Err($error)
    };
}

#[inline]
fn check_supported_format(mat: &impl MatTraitConst) -> Result<(), Error> {
    if mat.typ() != CV_8UC3 {
        bail!(Error::UnsupportedFormat)
    }
    Ok(())
}

#[inline]
fn check_and_get_dims(mat: &impl MatTraitConst) -> Result<(u32, u32), Error> {
    let w = mat.cols();
    if w <= 0 {
        bail!(Error::InvalidDimensions)
    }
    let h = mat.rows();
    if h <= 0 {
        bail!(Error::InvalidDimensions)
    }
    Ok((w as u32, h as u32))
}

#[inline]
fn full_check_and_get_dims(
    mat: &impl MatTraitConst,
) -> Result<(u32, u32), Error> {
    check_supported_format(mat)?;
    check_and_get_dims(mat)
}

fn new_rgb_image(mat: &impl MatTraitConst) -> Result<RgbImage, Error> {
    let (w, h) = full_check_and_get_dims(mat)?;
    Ok(RgbImage::new(w as u32, h as u32))
}

/// Represents anything that can be converted into DynamicImage
pub trait ToImage {
    /// Error in conversion
    type Err;

    /// Converts T to DynamicImage
    fn to_image(&self) -> Result<DynamicImage, Self::Err>;

    #[cfg(feature = "rayon")]
    /// Converts T to DynamicImage using rayon parallel iterators
    fn to_image_par(&self) -> Result<DynamicImage, Self::Err>;

    #[cfg(feature = "experimental")]
    fn as_image_buffer(
        &self,
    ) -> Result<ImageBuffer<custom_pix::Bgr, &[u8]>, Self::Err>;
}

impl<M> ToImage for M
where
    M: MatTraitConstManual,
{
    type Err = Error;

    fn to_image(&self) -> Result<DynamicImage, Error> {
        let mut rgbim = new_rgb_image(self)?;
        // pixels * 3 channels: already considered in rgbim.len() since it
        // derefs to [P::Subpixel], which is the primitive. See:
        // https://docs.rs/image/0.24.2/image/struct.ImageBuffer.html#deref-methods-%5BP%3A%3ASubpixel%5D
        let data = self.data_bytes()?;
        let w = rgbim.width();
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

    #[cfg(feature = "rayon")]
    fn to_image_par(&self) -> Result<DynamicImage, Self::Err> {
        let mut rgbim = new_rgb_image(self)?;
        // pixels * 3 channels: already considered in rgbim.len() since it
        // derefs to [P::Subpixel], which is the primitive. See:
        // https://docs.rs/image/0.24.2/image/struct.ImageBuffer.html#deref-methods-%5BP%3A%3ASubpixel%5D
        let data = self.data_bytes()?;

        use rayon::prelude::*;
        (&mut *rgbim)
            // .par_iter_mut()
            .par_chunks_mut(3)
            .zip(data.par_chunks(3))
            .for_each(|(rgbim_pix, mat_pix)| {
                let b = mat_pix[0];
                let g = mat_pix[1];
                let r = mat_pix[2];
                rgbim_pix[0] = r;
                rgbim_pix[1] = g;
                rgbim_pix[2] = b;
            });

        let im = DynamicImage::ImageRgb8(rgbim);
        Ok(im)
    }

    #[cfg(feature = "experimental")]
    fn as_image_buffer(
        &self,
    ) -> Result<ImageBuffer<custom_pix::Bgr, &[u8]>, Self::Err> {
        let (w, h) = full_check_and_get_dims(self)?;
        // pixels * 3 channels
        // let len = (w * h * 3) as usize;
        // let buf = slice::from_raw_parts(self.data(), len);
        // NO LONGER UNSAFE
        ImageBuffer::from_raw(w, h, self.data_bytes()?)
            .ok_or_else(|| Error::ContainerNotBigEnough)
    }
}

#[cfg(test)]
mod test {
    use opencv::imgcodecs::{imread, IMREAD_COLOR};

    use super::*;

    #[cfg(feature = "rayon")]
    #[test]
    fn serial_eq_par() {
        let mat = imread("examples/tinta_helada.jpg", IMREAD_COLOR)
            .expect("Failed to imread");
        let im1 = mat.to_image().expect("Failed to serially convert");
        let im2 = mat.to_image_par().expect("Failed to parallelly convert");

        assert_eq!(im1, im2)
    }
}

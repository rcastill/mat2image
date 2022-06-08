use image::{Pixel, PixelWithColorType, DynamicImage};

#[derive(Clone, Copy)]
pub struct Bgr(pub [u8; 3]);

// Implementation HEAVILY INSPIRED (almost copy paste) in
// https://docs.rs/image/latest/src/image/color.rs.html#358-370

/// Coefficients to transform from sRGB to a CIE Y (luminance) value.
const SRGB_LUMA: [u32; 3] = [2126, 7152, 722];
const SRGB_LUMA_DIV: u32 = 10000;

#[inline]
fn bgr_to_luma(&Bgr([b, g, r]): &Bgr) -> u8 {
    let l = SRGB_LUMA[0] * r as u32
        + SRGB_LUMA[1] * g as u32
        + SRGB_LUMA[2] * b as u32;
    (l / SRGB_LUMA_DIV) as u8
}

impl Pixel for Bgr {
    type Subpixel = u8;

    const CHANNEL_COUNT: u8 = 3;

    fn channels(&self) -> &[Self::Subpixel] {
        &self.0
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        &mut self.0
    }

    const COLOR_MODEL: &'static str = "BGR";

    fn channels4(
        &self,
    ) -> (
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
        Self::Subpixel,
    ) {
        (self.0[0], self.0[1], self.0[2], 255)
    }

    fn from_channels(
        a: Self::Subpixel,
        b: Self::Subpixel,
        c: Self::Subpixel,
        _d: Self::Subpixel,
    ) -> Self {
        Self([a, b, c])
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), 3);
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), 3);
        unsafe { &mut *(slice.as_mut_ptr() as *mut Self) }
    }

    fn to_rgb(&self) -> image::Rgb<Self::Subpixel> {
        let [b, g, r] = self.0;
        image::Rgb([r, g, b])
    }

    fn to_rgba(&self) -> image::Rgba<Self::Subpixel> {
        let [b, g, r] = self.0;
        image::Rgba([r, g, b, 255])
    }

    fn to_luma(&self) -> image::Luma<Self::Subpixel> {
        image::Luma([bgr_to_luma(self)])
    }

    fn to_luma_alpha(&self) -> image::LumaA<Self::Subpixel> {
        image::LumaA([bgr_to_luma(self), 255])
    }

    fn map<F>(&self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply(f);
        this
    }

    fn apply<F>(&mut self, mut f: F)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        for v in &mut self.0 {
            *v = f(*v)
        }
    }

    fn map_with_alpha<F, G>(&self, f: F, g: G) -> Self
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply_with_alpha(f, g);
        this
    }

    fn apply_with_alpha<F, G>(&mut self, mut f: F, mut g: G)
    where
        F: FnMut(Self::Subpixel) -> Self::Subpixel,
        G: FnMut(Self::Subpixel) -> Self::Subpixel,
    {
        const ALPHA: usize = 3 - 0;
        for v in self.0[..ALPHA].iter_mut() {
            *v = f(*v)
        }
        // The branch of this match is `const`. This way ensures that no subexpression fails the
        // `const_err` lint (the expression `self.0[ALPHA]` would).
        if let Some(v) = self.0.get_mut(ALPHA) {
            *v = g(*v)
        }
    }

    fn map2<F>(&self, other: &Self, f: F) -> Self
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        let mut this = (*self).clone();
        this.apply2(other, f);
        this
    }

    fn apply2<F>(&mut self, other: &Self, mut f: F)
    where
        F: FnMut(Self::Subpixel, Self::Subpixel) -> Self::Subpixel,
    {
        for (a, &b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = f(*a, b)
        }
    }

    fn invert(&mut self) {
        self.0[0] = 255 - self.0[0];
        self.0[1] = 255 - self.0[1];
        self.0[2] = 255 - self.0[2];
    }

    fn blend(&mut self, other: &Self) {
        *self = *other;
    }
}
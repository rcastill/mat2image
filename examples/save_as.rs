//! This example reads an image (examples/tinta_helada.jpg) using `opencv` and
//! saves it using `image` API.

use std::{borrow::Cow, env::args, time::Instant};

use mat2image::ToImage;
use opencv::imgcodecs::{imread, IMREAD_COLOR};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Read tinta_helada.jpg using opencv
    let start = Instant::now();
    let mat = imread("examples/tinta_helada.jpg", IMREAD_COLOR)?;
    let imread_elapsed = start.elapsed();

    // let start = Instant::now();
    // let mut mat2 = mat;
    // cvt_color(&mat, &mut mat2, COLOR_BGR2RGB, 0)?;
    // let mat2_elapsed = start.elapsed();
    // eprintln!("mat2 = {mat2_elapsed:?}");

    // Convert it to image::DynamicImage
    let start = Instant::now();
    let im = mat.to_image()?;
    let conv_elapsed = start.elapsed();

    #[cfg(feature = "rayon")]
    let conv_par_elapsed = {
        let start = Instant::now();
        let _im_par = mat.to_image_par()?;
        start.elapsed()
    };

    // Convert it to ImageBuffer using unsafe (fast + no allocations)
    #[cfg(feature = "experimental")]
    let conv_noalloc_elapsed = {
        let start = Instant::now();
        let _im = mat.as_image_buffer()?;
        start.elapsed()
    };

    // Write file to output provided by user or default out.jpg using `image`
    // API
    let outfile = args()
        .nth(1)
        .map(|mut out| {
            // do not overwrite tinta_helada
            if out == "examples/tinta_helada.jpg" {
                eprintln!("Refusing to overwrite tinta_helada.jpg: writing to out.jpg");
                return Cow::Borrowed("out.jpg");
            }
            // Filename should always end in .jpg
            if !out.ends_with(".jpg") {
                out += ".jpg";
            }
            Cow::Owned(out)
        })
        .unwrap_or_else(|| Cow::Borrowed("out.jpg"));
    let start = Instant::now();
    im.save(&*outfile)?;
    let save_elapsed = start.elapsed();

    // test
    eprintln!("imread   : {imread_elapsed:?}");
    eprintln!("conv     : {conv_elapsed:?}");
    #[cfg(feature = "rayon")]
    eprintln!("conv_par : {conv_par_elapsed:?}");
    #[cfg(feature = "experimental")]
    eprintln!("conv-noa : {conv_noalloc_elapsed:?}");
    eprintln!("save     : {save_elapsed:?}");
    Ok(())
}

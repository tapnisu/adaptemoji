pub mod cli;

/// Converts the pixels of a GrayAlphaImage to negative if `negative` is true,
/// otherwise, converts them to positive.
///
/// For Telegram's adaptive emojis, it doesn't matter, made to better show
/// what should you use depending on the background.
///
/// # Arguments
///
/// * `img` - A mutable reference to a GrayAlphaImage.
/// * `negative` - A boolean indicating whether to convert to negative or not.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), image::ImageError> {
/// let mut img = image::open("./assets/examples/original.webp")?.to_luma_alpha8();
///
/// adaptemoji::convert(&mut img, true);
/// # Ok(())
/// # }
/// ```
pub fn convert(img: &mut image::GrayAlphaImage, negative: bool) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);

            // For Telegram's adaptive emojis, it doesn't matter, made to better show what should you use depending on background
            let new_color = if negative { 0 } else { 255 };
            let new_alpha = (((if negative {
                255 - pixel.0[0]
            } else {
                pixel.0[0]
            }) as f32
                / 255f32)
                * pixel.0[1] as f32) as u8;

            let proc_pixel = image::LumaA::from([new_color, new_alpha]);
            img.put_pixel(x, y, proc_pixel);
        }
    }
}

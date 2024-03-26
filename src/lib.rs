//! # Adaptemoji
//!
//! Convert your regular Telegram emojis into adaptive monochrome versions
//!
//! ## Installation as CLI
//!
//! ```bash
//! cargo install adaptemoji
//! ```
//!
//! ## How to use
//!
//! ### Regular
//!
//! ```bash
//! adaptemoji -i your-image.png -o output-image.png
//! ```
//!
//! ### Negative
//!
//! ```bash
//! adaptemoji -i your-image.png -o output-image.png -n
//! ```
//!
//! Also Telegram requires your emoji to be 100px x 100px in size. If you want adaptemoji automatically to resize image, add `-r` flag
//!
//! ### Regular resized
//!
//! ```bash
//! adaptemoji -i your-image.png -o output-image.png -r
//! ```
//!
//! ### Negative resized
//!
//! ```bash
//! adaptemoji -i your-image.png -o output-image.png -nr
//! ```
//!
//! ## Using as library
//!
//! ### Installation
//!
//! ```bash
//! cargo add adaptemoji
//! ```
//!
//! ### Examples
//!
//! ```rust
//! use adaptemoji::AdaptiveEmojiConvert;
//! use std::error;
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     let img = image::open("./assets/examples/original.webp")?;
//!     let mut resized_img = img
//!         .resize_to_fill(100, 100, image::imageops::FilterType::Triangle)
//!         .to_luma_alpha8();
//!
//!     resized_img.convert(false).save("./target/adaptive.png")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ```rust
//! use adaptemoji::AdaptiveEmojiConvert;
//! use std::error;
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     let img = image::open("./assets/examples/original.webp")?;
//!     let mut resized_img = img
//!         .resize_to_fill(100, 100, image::imageops::FilterType::Triangle)
//!         .to_luma_alpha8();
//!
//!     adaptemoji::convert(&mut resized_img, true);
//!     resized_img.save("./target/adaptive_negative.png")?;
//!
//!     Ok(())
//! }
//! ```

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

/// Convert your emoji into adaptive one
pub trait AdaptiveEmojiConvert {
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
    /// use adaptemoji::AdaptiveEmojiConvert;
    ///
    /// # fn main() -> Result<(), image::ImageError> {
    /// let mut img = image::open("./assets/examples/original.webp")?.to_luma_alpha8();
    ///
    /// let adaptive_img = img.convert(true);
    /// # Ok(())
    /// # }
    /// ```
    fn convert(&self, negative: bool) -> Self;
}

impl AdaptiveEmojiConvert for image::GrayAlphaImage {
    fn convert(&self, negative: bool) -> image::GrayAlphaImage {
        let mut img = self.clone();
        convert(&mut img, negative);
        img
    }
}

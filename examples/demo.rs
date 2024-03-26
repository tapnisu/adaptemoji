use adaptemoji::AdaptiveEmojiConvert;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let img = image::open("./assets/examples/original.webp")?;
    let mut resized_img = img
        .resize_to_fill(100, 100, image::imageops::FilterType::Triangle)
        .to_luma_alpha8();

    resized_img.convert_adaptive(false).save("./target/adaptive.png")?;

    adaptemoji::convert_adaptive(&mut resized_img, true);
    resized_img.save("./target/adaptive_negative.png")?;

    Ok(())
}

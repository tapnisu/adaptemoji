pub mod cli;

pub fn convert(img: &mut image::GrayAlphaImage, negative: bool) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);

            let new_alpha = (((if negative {
                255 - pixel.0[0]
            } else {
                pixel.0[0]
            }) as f32
                / 255f32)
                * pixel.0[1] as f32) as u8;

            let proc_pixel = image::LumaA::from([255, new_alpha]);
            img.put_pixel(x, y, proc_pixel);
        }
    }
}

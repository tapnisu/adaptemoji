pub mod cli;

pub fn convert(img: &mut image::GrayAlphaImage) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let proc_pixel = image::LumaA::from([
                255,
                (((255 - pixel.0[0]) as f32 / 255f32) * pixel.0[1] as f32) as u8,
            ]);

            img.put_pixel(x, y, proc_pixel);
        }
    }
}

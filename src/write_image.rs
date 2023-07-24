use image::{ImageBuffer, Rgb};

use crate::{
    buffer::Buffer,
    color::Color,
    constants::{RESOLUTION_X, RESOLUTION_Y},
};

/**
 * Write a frame to a PNG file
 */
pub fn write_image(buffer: Buffer<Color>, file: &str) {
    println!("Writing to png...");

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(RESOLUTION_X as u32, RESOLUTION_Y as u32);

    for x in 0..RESOLUTION_X {
        for y in 0..RESOLUTION_Y {
            let color = buffer[x][y];

            image.get_pixel_mut(x as u32, y as u32).0 = [
                (color.0 * 255.0) as u8,
                (color.1 * 255.0) as u8,
                (color.2 * 255.0) as u8,
            ];
        }
    }

    image.save(file).unwrap();

    println!("done");
}

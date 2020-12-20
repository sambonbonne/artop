use std::vec::Vec;
use image::{RgbImage, Rgb};


pub type Pixel = Rgb<usize>;
pub type Pixels = Vec<Pixel>;
pub type Filler = fn(&Pixels, &mut RgbImage);

pub fn write(pixels: &Pixels, filler: Filler, dest: String) {
    let diagonal_pixels_amount = pixels.len() as u32;
    let mut image_buffer = RgbImage::new(diagonal_pixels_amount, diagonal_pixels_amount);

    filler(pixels, &mut image_buffer);

    image_buffer.save(dest).unwrap();
}

pub mod fillers {
    use super::{ Pixels, RgbImage, Rgb };

    pub fn smooth(pixels: &Pixels, image_buffer: &mut RgbImage) {
        for (i, colors) in pixels.iter().enumerate() {
            let pixel = image_buffer.get_pixel_mut(i as u32, i as u32);
            *pixel = Rgb([colors[0] as u8, colors[1] as u8, colors[2] as u8]);
        }

        let max_size = image_buffer.width() - 1;

        for i in 0u32..max_size {
            for j in 0u32..(max_size-i) {
                let image_buffer_reader = image_buffer.clone();

                let current_pixel = image_buffer_reader.get_pixel(j + i, j);
                let next_pixel = image_buffer_reader.get_pixel(j + i + 1, j + 1);

                let new_color = Rgb([
                    ((current_pixel.0[0] as f64 + next_pixel.0[0] as f64) / 2.0) as u8,
                    ((current_pixel.0[1] as f64 + next_pixel.0[1] as f64) / 2.0) as u8,
                    ((current_pixel.0[2] as f64 + next_pixel.0[2] as f64) / 2.0) as u8
                ]);

                image_buffer.put_pixel(j + i + 1, j, new_color);
                image_buffer.put_pixel(j, j + i + 1, new_color);
            }
        }
    }

    pub fn gradient(pixels: &Pixels, image_buffer: &mut RgbImage) {
        let max_size = image_buffer.width() - 1;

        for (i, colors) in pixels.iter().enumerate() {
            let i = i as u32;

            let current_color = Rgb([
                colors[0] as u8,
                colors[1] as u8,
                colors[2] as u8]
            );

            image_buffer.put_pixel(i, i, current_color);

            if i > 0 {
                let previous_pixel = image_buffer.get_pixel(i - 1, i - 1);
                let current_pixel = image_buffer.get_pixel(i, i);

                let new_color = Rgb([
                    ((previous_pixel.0[0] as f64 + current_pixel.0[0] as f64) / 2.0) as u8,
                    ((previous_pixel.0[1] as f64 + current_pixel.0[1] as f64) / 2.0) as u8,
                    ((previous_pixel.0[2] as f64 + current_pixel.0[2] as f64) / 2.0) as u8
                ]);

                let range = if i <= max_size / 2 {
                        0u32..i
                } else if i < max_size {
                        0u32..(max_size - i + 1)
                } else {
                    0u32..1u32
                };

                for j in range {
                    image_buffer.put_pixel(i + j, i - 1 - j, new_color);
                    image_buffer.put_pixel(i - 1 - j, i + j, new_color);
                    image_buffer.put_pixel(i + j, i - j, current_color);
                    image_buffer.put_pixel(i - j, i + j, current_color);
                }

                if i <= max_size / 2 {
                    image_buffer.put_pixel(i*2, 0, current_color);
                    image_buffer.put_pixel(0, i*2, current_color);
                }
            }
        }
    }
}

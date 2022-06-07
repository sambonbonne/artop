use std::{io, thread::sleep, time::Duration, vec::Vec};

use crate::collect::StatReader;
use crate::write::{write, Filler};

use image::Rgb;

struct ColorReaders<'a, T: StatReader, U: StatReader, V: StatReader> {
    red: &'a T,
    green: &'a U,
    blue: &'a V,
}

pub struct Converter<'a, T: StatReader, U: StatReader, V: StatReader> {
    readers: ColorReaders<'a, T, U, V>,
    pixels: Vec<Rgb<usize>>,
}

impl<'a, T, U, V> Converter<'a, T, U, V>
where
    T: StatReader,
    U: StatReader,
    V: StatReader,
{
    pub fn new<'b, W: StatReader, X: StatReader, Y: StatReader>(
        red_reader: &'b W,
        green_reader: &'b X,
        blue_reader: &'b Y,
    ) -> Converter<'b, W, X, Y> {
        Converter {
            readers: ColorReaders {
                red: red_reader,
                blue: blue_reader,
                green: green_reader,
            },
            pixels: Vec::new(),
        }
    }

    pub fn run(&mut self, rounds: u32, sleep_ms: u64) -> io::Result<bool> {
        for _ in 0..rounds {
            let red_value = self.readers.red.read(255.0)?;
            let green_value = self.readers.green.read(255.0)?;
            let blue_value = self.readers.blue.read(255.0)?;

            self.pixels.push(Rgb([
                red_value as usize,
                green_value as usize,
                blue_value as usize,
            ]));

            sleep(Duration::from_millis(sleep_ms));
        }

        Ok(true)
    }

    pub fn finish(&mut self, fill_method: Filler, output_path: String) {
        write(&self.pixels, fill_method, output_path);
    }
}

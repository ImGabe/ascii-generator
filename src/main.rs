use core::f32;
use image::{buffer::Pixels, imageops::FilterType};
use image::Luma;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::{fs::File, io::Write, path::Path};

// TO-DO: transform into variables for the user
const PATH: &str = "assets/433.png";
const BRIGHT: i32 = 100;
const SIZE: usize = 256;

const ASCII_CHARS: [char; 69] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w',
    'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j',
    'f', 't', '/', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>', 'i',
    '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' ',
];

fn open_image(path: &str) -> DynamicImage {
    image::open(Path::new(path)).expect("Not a valid image path or could no open image")
}

fn modify_image(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let ratio = height as f32 / width as f32;
    let new_width = SIZE as f32;
    let new_height = ratio * new_width;

    image.brighten(BRIGHT)
        .resize_exact(new_width as u32, new_height as u32, FilterType::Nearest)
        .to_luma8()
}

fn pixel_to_ascii(pixel: Luma<u8>) -> char {
    let level_of_bright = (pixel[0] as usize) * (ASCII_CHARS.len() - 1) / 255;
    ASCII_CHARS[level_of_bright]
}

fn pixels_to_ascii(pixels: Pixels<Luma<u8>>) -> String {
    let ascii: Vec<char> = pixels.map(|p| pixel_to_ascii(*p)).collect();
    let ascii_vec: Vec<String> = ascii
        .chunks(SIZE)
        .map(|line| line.iter().cloned().collect::<String>())
        .collect();

    ascii_vec.join("\n")
}

fn to_file(buffer: &[u8]) {
    let mut file = File::create("ascii_art.txt").expect("Can't create file.");
    file.write_all(buffer).expect("Can't write file.");
}

fn main() {
    let img = open_image(PATH);
    let imgbuf = modify_image(img);
    let pixels = imgbuf.pixels();
    let ascii_art = pixels_to_ascii(pixels);

    to_file(ascii_art.as_bytes());
    println!("{}", ascii_art);
}

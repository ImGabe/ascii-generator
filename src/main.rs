use image::Luma;
use image::{buffer::Pixels, imageops::FilterType};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::{fs::File, io::Write, path::Path};

use std::path::PathBuf;
use structopt::StructOpt;

/// Transform images to ASCII
#[derive(StructOpt, Debug)]
#[structopt(name = "Ascii-generator")]
struct Opt {
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Invert color file
    #[structopt(short, long)]
    invert: bool,

    /// Output size
    #[structopt(short, long)]
    size: usize,

    /// Output right
    #[structopt(short, long, default_value = "0")]
    bright: i32,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

const ASCII_CHARS: [char; 69] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w',
    'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j',
    'f', 't', '/', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>', 'i',
    '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' ',
];

fn open_image(path: PathBuf) -> DynamicImage {
    image::open(Path::new(&path)).expect("Not a valid image path or could no open image")
}

fn modify_image(image: DynamicImage, size: usize, bright: i32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let ratio = height as f32 / width as f32;
    let new_width = size as f32;
    let new_height = ratio * new_width;

    image
        .brighten(bright)
        .resize_exact(new_width as u32, new_height as u32, FilterType::Nearest)
        .to_luma8()
}

fn pixel_to_ascii(pixel: Luma<u8>, invert: bool) -> char {
    let ascii_chars_length = ASCII_CHARS.len();
    let level_of_bright = (pixel[0] as usize) * (ascii_chars_length - 1) / 255;

    if invert {
        ASCII_CHARS[ascii_chars_length - level_of_bright]
    } else {
        ASCII_CHARS[level_of_bright]
    }    
}

fn pixels_to_ascii(pixels: Pixels<Luma<u8>>, size: usize, invert: bool) -> String {
    let ascii: Vec<char> = pixels.map(|p| pixel_to_ascii(*p, invert)).collect();
    let ascii_vec: Vec<String> = ascii
        .chunks(size)
        .map(|line| line.iter().cloned().collect::<String>())
        .collect();

    ascii_vec.join("\n")
}

fn to_file(buffer: &[u8], output: PathBuf) {
    let mut file = File::create(output).expect("Can't create file.");
    file.write_all(buffer).expect("Can't write file.");
}

fn main() {
    let opt = Opt::from_args();

    let img = open_image(opt.file);
    let imgbuf = modify_image(img, opt.size, opt.bright);
    let pixels = imgbuf.pixels();
    let ascii_art = pixels_to_ascii(pixels, opt.size, opt.invert);

    if let Some(output) = &opt.output {
        to_file(ascii_art.as_bytes(), output.to_path_buf());
    }

    println!("{}", ascii_art);
}

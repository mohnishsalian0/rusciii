use clipboard::{ClipboardContext, ClipboardProvider};
use image::{GenericImageView, GrayImage, RgbImage};
use leptos::svg::image;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Char {
    intensity: u8,
    deviation: u8,
    id: u8,
}

#[derive(Serialize, Deserialize)]
struct Font {
    name: String,
    charIntensity: Vec<u8>,
    charDeviation: Vec<u8>,
}

pub struct AsciiGenerator {
    fonts: Vec<Font>,
}

impl AsciiGenerator {
    pub fn new() -> Self {
        // let fontPath = PathBuf::from("data/fonts.json");
        // let file = File::open(fontPath).expect("Failed to open font file");
        // let reader = BufReader::new(file);
        // let fonts: Vec<Font> = serde_json::from_reader(reader).expect("Failed to parse fonts json");
        let jsonData = include_str!("../data/fonts.json");
        let fonts: Vec<Font> = serde_json::from_str(jsonData).expect("Failed to parse fonts json");
        Self { fonts }
    }

    fn getChars(&self, font: &str) -> Vec<Char> {
        let Font {
            name: _,
            charIntensity: ci,
            charDeviation: ca,
        } = self
            .fonts
            .iter()
            .find(|f| f.name == font)
            .expect("Font {font} not found");

        let mut chars: Vec<Char> = (32..127)
            .map(|id| Char {
                intensity: ci[id - 32],
                deviation: ca[id - 32],
                id: id.try_into().unwrap(),
            })
            .collect();
        chars.sort();

        // Retain chars with unique intensities based on lowest deviation
        let mut uniqueIntens = HashSet::new();
        chars.retain(|c| uniqueIntens.insert(c.intensity));

        // Scale intensity range to 0-255
        let (minI, maxI) = (chars[0].intensity, chars[chars.len() - 1].intensity);
        let range: f32 = (maxI - minI).into();
        chars.iter_mut().for_each(|c| {
            c.intensity = ((c.intensity - minI) as f32 * 255.0 / range).round() as u8
        });
        chars
    }

    fn getWeightedRamp(&self, font: &str, filterChars: &str) -> [u8; 256] {
        let mut chars = self.getChars(font);
        chars.retain(|c| {
            let char = char::from(c.id);
            filterChars.contains(char)
        });

        // Build ascii ramp
        let mut asciiRamp: [u8; 256] = [chars[0].id; 256];
        let mut i: usize = 1;
        for j in 1..chars.len() {
            let mut mid = ((chars[j - 1].intensity as u16 + chars[j].intensity as u16) / 2) as u8;
            while i <= chars[j].intensity.into() {
                asciiRamp[i] = if i < mid.into() {
                    chars[j - 1].id
                } else {
                    chars[j].id
                };
                i += 1;
            }
        }
        asciiRamp
    }

    pub fn convert(&self, font: &str, chars: &str, img: &GrayImage) -> Vec<Vec<u8>> {
        let ramp = self.getWeightedRamp(font, chars);
        let (w, h) = img.dimensions();
        let mut asciiArt: Vec<Vec<u8>> = vec![vec![0; w as usize]; h as usize];
        for (x, y, p) in img.enumerate_pixels() {
            asciiArt[y as usize][x as usize] = ramp[p[0] as usize];
        }
        asciiArt
    }
}

pub fn addAnsiTrueColor(art: &[Vec<u8>], img: &RgbImage) -> Vec<Vec<String>> {
    let (w, h) = img.dimensions();
    let (w, h) = (w as usize, h as usize);
    let mut coloredArt = vec![vec![String::new(); w]; h];
    for (i, row) in coloredArt.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let p = img.get_pixel(j as u32, i as u32);
            let (r, g, b) = (p[0], p[1], p[2]);
            *val = format!("\x1B[38;2;{};{};{}m{}", r, g, b, char::from(art[i][j]));
        }
    }
    coloredArt
}

pub fn to_string(art: &[Vec<u8>]) -> String {
    String::from_utf8_lossy(&art.join(&10)).to_string()
}

pub fn display(art: &Vec<Vec<u8>>) {
    for row in art {
        let strRow = String::from_utf8_lossy(row);
        println!("{strRow}");
    }
}

pub fn displayColored(art: &Vec<Vec<String>>) {
    for row in art {
        let strRow = row.join("");
        println!("{strRow}");
    }
}

pub fn copyToClipboard(art: &[Vec<u8>]) {
    let textToCopy = art
        .iter()
        .map(|row| String::from_utf8_lossy(row).to_string())
        .collect::<Vec<String>>()
        .join("\n");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(textToCopy.to_owned()).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::imageHandler::ImageHandler;

    use super::*;
    use image::io::Reader as ImageReader;
    use image::{imageops::FilterType, GenericImageView};
    use image::{GrayImage, Luma};
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_get_chars() {
        let ascii_gen = AsciiGenerator::new();
        let font_name = "menlo";
        let chars = ascii_gen.getChars(font_name);
        assert_eq!(chars[0].intensity, 0);
        assert_eq!(chars[chars.len() - 1].intensity, 255);
    }

    #[test]
    fn test_get_weighted_ramp() {
        let ascii_gen = AsciiGenerator::new();
        let font_name = "menlo";
        let chars = ascii_gen.getChars(font_name);
        let weighted_ramp = ascii_gen.getWeightedRamp(font_name, "chars");
        println!("Generated ramp: {weighted_ramp:?}");
        assert_eq!(weighted_ramp.len(), 256);
    }

    #[test]
    fn test_convert() {
        use std::time::Instant;
        let now = Instant::now();

        let font = "menlo".to_string();

        // let chars = "5,;AsrS3.&hX# 2M@9:BiGH".to_string();
        let chars = "@#MBHA&Gh93X25Sisr;:,. ".to_string();

        let imagePath = PathBuf::from("assets/testImage4.jpeg");
        let img = ImageReader::open(imagePath).expect("Image not found");
        let img = img.decode().expect("Failed to decode image");
        let (w, h) = img.dimensions();
        let nw = 1000;
        let nh = nw * h / w;
        let mut img = img
            .adjust_contrast(16.1)
            .resize(nw, nh, FilterType::Nearest);
        let imgRgb = img.to_rgb8();
        let gray = img.into_luma8().downsample().stretchContrast();

        let ascGen = AsciiGenerator::new();

        let asciiArt = ascGen.convert(&font, &chars, &gray);

        display(&asciiArt);

        // let asciiArt = addAnsiTrueColor(&asciiArt, &imgRgb);

        // displayColored(&asciiArt);

        // copyToClipboard(&asciiArt);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}

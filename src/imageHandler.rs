use image::{
    imageops::dither, DynamicImage, GenericImage, GenericImageView, GrayImage, Luma, RgbImage,
};
use leptos::*;

pub trait ImageHandler {
    fn downsample(self) -> Self;
    fn stretchContrast(self) -> Self;
    fn dither(&mut self);
}

impl ImageHandler for GrayImage {
    fn downsample(self) -> Self {
        let (w, h) = self.dimensions();
        let (fw, fh) = (7, 14);
        let (w, h) = (((w + fw - 1) / fw) as usize, ((h + fh - 1) / fh) as usize);
        let mut avgIntens = vec![vec![(0_u32, 0_u32); w]; h];
        for (x, y, p) in self.enumerate_pixels() {
            let (nx, ny) = ((x / fw) as usize, (y / fh) as usize);
            avgIntens[ny][nx].0 += p[0] as u32;
            avgIntens[ny][nx].1 += 1;
        }

        let mut res = GrayImage::new(w as u32, h as u32);
        avgIntens.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, &(i, c))| {
                res.put_pixel(
                    y as u32,
                    x as u32,
                    Luma([(i as f64 / c as f64).round() as u8]),
                );
            })
        });

        res
    }

    fn stretchContrast(mut self) -> Self {
        let (mut minP, mut maxP): (u8, u8) = (255, 0);
        self.iter().for_each(|&p| {
            minP = minP.min(p);
            maxP = maxP.max(p);
        });
        let rangeP = maxP - minP;
        self.enumerate_pixels_mut().for_each(|(x, y, p)| {
            p[0] = ((p[0] - minP) as f32 * 255.0 / rangeP as f32).round() as u8
        });
        self
    }

    fn dither(&mut self) {
        let colorMap: Vec<u8> = (0..=255).collect();
        // dither(self, &colorMap)
    }
}

pub fn grayscale(img: &RgbImage) -> Vec<Vec<u8>> {
    let (w, h) = img.dimensions();
    let mut result = vec![vec![0; w as usize]; h as usize];
    for (x, y, p) in img.enumerate_pixels() {
        let (r, g, b) = (p[0] as u32, p[1] as u32, p[2] as u32);
        let lum = ((2126 * r + 7152 * g + 722 * b) as f64 / 10000.0).round() as u8;
        result[y as usize][x as usize] = lum;
    }
    result
}

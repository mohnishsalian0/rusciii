use image::{DynamicImage, GenericImageView, GrayImage, RgbImage};

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

pub fn downsample(img: &[Vec<u8>], w: u32, h: u32) -> Vec<Vec<u8>> {
    let mut avgIntensity: Vec<Vec<u32>> = vec![vec![0; w as usize]; h as usize];
    for (x, row) in img.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            let (nx, ny) = (x / 14, y / 7);
            avgIntensity[nx][ny] += value as u32;
        }
    }

    avgIntensity
        .iter()
        .map(|row| {
            row.iter()
                .map(|&i| (i as f64 / 98.0).round() as u8)
                .collect()
        })
        .collect()
}

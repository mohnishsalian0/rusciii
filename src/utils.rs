use image::{imageops::ColorMap, Luma};

pub struct AsciiColorMap {
    colorMap: [u8; 256],
}

impl AsciiColorMap {
    pub fn new(colorMap: [u8; 256]) -> Self {
        AsciiColorMap { colorMap }
    }
}

impl ColorMap for AsciiColorMap {
    type Color = Luma<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        color[0] as usize
    }

    fn map_color(&self, color: &mut Self::Color) {
        let newColor = self.index_of(color);
        let luma = &mut color.0;
        luma[0] = self.colorMap[newColor];
    }
}

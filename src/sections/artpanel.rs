use std::path::PathBuf;

use image::{
    imageops::{dither, FilterType},
    io::Reader as ImageReader,
    DynamicImage, GenericImageView, GrayImage,
};
use leptos::*;

use crate::{
    app::Test,
    asciiGenerator::{self, to_string, AsciiGenerator},
    components::{select::*, slider::*, upload::*},
    utils::AsciiColorMap,
};

#[component]
pub fn ArtPanel<F>(image: F, dither: ReadSignal<bool>) -> impl IntoView
where
    F: Fn() -> Option<GrayImage> + 'static,
{
    let font = "menlo".to_string();
    let chars = "@#MBHA&Gh93X25Sisr;:,. ".to_string();
    let ascGen = AsciiGenerator::new();
    let (intensityMap, charMap) = ascGen.getIntensityDistAndCharMap(&font, &chars);
    let ascColorMap = AsciiColorMap::new(intensityMap);
    let asciiArt = move || {
        let art: Vec<Vec<u8>> = match image().as_ref() {
            Some(img) => match dither() {
                true => ascGen.convertWithDither(&font, &chars, img),
                false => ascGen.convert(&font, &chars, img),
            },
            None => vec![vec![0; 1]; 1],
        };
        to_string(&art)
    };

    view! {
        <div class="flex-1 h-auto p-8 bg-transparent text-slate-700 overflow-auto font-mono text-sm leading-4">
            <pre>{asciiArt}</pre>
        </div>
    }
}

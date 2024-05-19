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
    let font = "courierPrime".to_string();
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
            None => {
                vec![0_u8; 0];
                vec![] as Vec<Vec<u8>>
            }
        };
        to_string(&art)
    };

    view! {
        <div class="flex-1 h-auto p-8 bg-transparent overflow-auto">
            <pre class="font-['Courier_New'] text-slate-700 font-mono text-sm leading-4">{asciiArt}</pre>
        </div>
    }
}

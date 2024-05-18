use std::path::PathBuf;

use image::{
    imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView, GrayImage,
};
use leptos::*;

use crate::{
    app::Test,
    asciiGenerator::{self, to_string, AsciiGenerator},
    components::{select::*, slider::*, upload::*},
};

#[component]
pub fn ArtPanel<F>(image: F) -> impl IntoView
where
    F: Fn() -> Option<GrayImage> + 'static,
{
    let ascGen = AsciiGenerator::new();
    let asciiArt = move || {
        let font = "menlo".to_string();
        let chars = "@#MBHA&Gh93X25Sisr;:,. ".to_string();
        let art = match image().as_ref() {
            Some(img) => ascGen.convert(&font, &chars, img),
            None => vec![vec![0; 1]; 1],
        };
        to_string(&art)
    };

    view! {
        <div class="flex-1 h-auto p-8 bg-transparent text-slate-700 overflow-auto font-mono text-sm leading-4">
            <pre>
                {asciiArt}
            </pre>
        </div>
    }
}

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
pub fn ArtPanel<F>(grayImage: F) -> impl IntoView
where
    F: Fn() -> Option<GrayImage> + 'static,
{
    let ascGen = AsciiGenerator::new();
    let asciiArt = move || {
        let font = "menlo".to_string();
        let chars = "5,;AsrS3.&hX# 2M@9:BiGH".to_string();
        let art = match grayImage() {
            Some(img) => ascGen.convert(&font, &chars, &img),
            None => vec![vec![0; 1]; 1],
        };
        to_string(&art)
    };

    view! {
        <div class="flex-1 h-auto p-4 bg-neutral-50 text-slate-700 rounded-xl border-2 border-neutral-200 ring-1 ring-neutral-100 overflow-auto font-mono text-sm leading-4">
            <pre>
                {asciiArt}
            </pre>
        </div>
    }
}

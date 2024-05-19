use image::{
    imageops::{self, brighten},
    DynamicImage, GrayImage,
};
use leptos::*;

use crate::{
    imageHandler::ImageHandler,
    sections::{artpanel::*, sidebar::*},
};

#[derive(Clone)]
pub struct Test {
    n: i32,
}

impl Test {
    pub fn echo(&self) {
        logging::log!("can call function from context");
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (image, setImage) = create_signal::<Option<DynamicImage>>(None);
    let (gray, setGray) = create_signal::<Option<GrayImage>>(None);
    let (contrast, setContrast) = create_signal::<f32>(0.0);
    let (brightness, setBrightness) = create_signal::<i32>(0);
    let (size, setSize) = create_signal::<u16>(100);
    let (resizedImage, setResizedImage) = create_signal::<Option<GrayImage>>(None);
    let (dither, setDither) = create_signal::<bool>(false);
    let filteredImage = move || {
        with!(
            |resizedImage, contrast, brightness| resizedImage.as_ref().map(|img| brighten(
                &imageops::contrast(img, *contrast),
                *brightness
            )
            .stretchContrast())
        )
    };

    view! {
        <div class="w-full h-full flex flex-row divide-x divide-amber-500">
            <Sidebar setImage gray setGray setResizedImage setContrast setBrightness setDither/>
            <ArtPanel image=filteredImage dither=dither/>
        </div>
    }
}

use base64::{self, engine::general_purpose, Engine};
use image::{
    imageops::{brighten, contrast, resize, FilterType},
    DynamicImage, GenericImageView, GrayImage, ImageFormat,
};
use leptos::*;
use regex::Regex;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    js_sys::{ArrayBuffer, Uint8Array},
    Event, FileReader, HtmlInputElement, MouseEvent, Url,
};

use crate::{
    components::{select::*, slider::*, toggle::*, upload::*},
    imageHandler::ImageHandler,
};

#[component]
pub fn Sidebar(
    setImage: WriteSignal<Option<DynamicImage>>,
    gray: ReadSignal<Option<GrayImage>>,
    setGray: WriteSignal<Option<GrayImage>>,
    setResizedImage: WriteSignal<Option<GrayImage>>,
    setContrast: WriteSignal<f32>,
    setBrightness: WriteSignal<i32>,
    setDither: WriteSignal<bool>,
) -> impl IntoView {
    let uploadRef: NodeRef<html::Input> = create_node_ref();

    let (imageUrl, setImageUrl) = create_signal::<Option<String>>(None);

    let displayImage = move || {
        imageUrl().map(|_| {
            // TODO: Add option to delete image and start over
            Some(
                view! { <img src=imageUrl alt="Unable to display uploaded image" class="w-full"/> },
            )
        })
    };

    let onImageUpload = move |_| {
        logging::log!("Uploading new image...");
        let file = uploadRef()
            .unwrap()
            .files()
            .unwrap()
            .get(0)
            .expect("No image uploaded");

        setImageUrl(Url::create_object_url_with_blob(&file).ok());

        let reader = FileReader::new().expect("Failed to create file reader");
        let frc = reader.clone();
        let onloadClosure = Closure::wrap(Box::new(move || {
            let arrayBuffer = frc.result().expect("Failed to read buffer");
            let uint8Array = Uint8Array::new(&arrayBuffer);
            let bytes: Vec<u8> = uint8Array.to_vec();

            let img = image::load_from_memory(&bytes)
                .expect("Failed to convert Uint8Array to DynamicImage");

            // TODO: Plugin filter states to modify image. Reset filter states when a new image is uploaded
            // image.set(Some(img.resize(nw, nh, FilterType::Nearest).clone()));
            let gray = img.into_luma8();
            setGray(Some(gray.clone()));

            let (w, h) = gray.dimensions();
            let nw = 700;
            let nh = nw * h / w;
            setResizedImage(Some(
                resize(&gray, nw, nh, FilterType::Nearest).downsample(),
            ));
        }) as Box<dyn FnMut()>);

        reader
            .read_as_array_buffer(&file)
            .expect("Failed to load image into buffer");
        reader.set_onload(Some(onloadClosure.as_ref().unchecked_ref()));
        onloadClosure.forget();
    };

    let displayInput = move || {
        imageUrl().map_or_else(
            || Some(view! { <Upload onUpload=onImageUpload inputRef=uploadRef/> }),
            |_| None,
        )
    };

    let onContrastChange = move |e: Event| {
        // TODO: Adjust contrast inplace instead of creating new image
        let c: f32 = event_target_value(&e)
            .parse()
            .expect("Contrast should be a number");
        logging::log!("Adjusting contrast to {:?}...", &c);
        setContrast(c);
    };

    let onBrightnessChange = move |e: Event| {
        // TODO: Adjust brightness in place instead of creating a new image
        let b: i32 = event_target_value(&e)
            .parse()
            .expect("Contrast should be a number");
        logging::log!("Adjusting brightness to {:?}...", &b);
        setBrightness(b);
    };

    let onSizeChange = move |e: Event| {
        let mut nw: u32 = event_target_value(&e)
            .parse()
            .expect("Contrast should be a number");
        nw *= 7;
        let (w, h) = gray().expect("Gray image should be set").dimensions();
        let nh = nw * h / w;
        let filter = if nw > w {
            FilterType::Lanczos3
        } else {
            FilterType::Triangle
        };
        logging::log!("Resizing to {} x {}...", nw, nh);
        setResizedImage(
            gray()
                .as_ref()
                .map(|img| resize(img, nw, nh, filter).downsample()),
        );
    };

    let onDitherChange = move |e: Event| {
        let c = event_target_checked(&e);
        if c {
            logging::log!("Adding dither...");
        } else {
            logging::log!("Removing dither...");
        }
        setDither(c);
    };

    view! {
        <aside class="w-80 h-auto bg-amber-50 overflow-y-auto">
            {displayImage}
            {displayInput}
            <div class="p-8 display-flex flex-col space-y-8">
                // <Select name="font".to_string() label="Font".to_string()/>
                <Slider
                    name="contrast".to_string()
                    label="Contrast".to_string()
                    min=-100
                    max=100
                    value=0
                    onInput=onContrastChange
                />
                <Slider
                    name="brightness".to_string()
                    label="Brightness".to_string()
                    min=-100
                    max=100
                    value=0
                    onInput=onBrightnessChange
                />
                <Slider
                    name="size".to_string()
                    label="Size".to_string()
                    min=10
                    max=300
                    value=100
                    onInput=onSizeChange
                />
                <Toggle
                    name="dither".to_string()
                    label="Dither".to_string()
                    onInput=onDitherChange
                />
            </div>
        </aside>
    }
}

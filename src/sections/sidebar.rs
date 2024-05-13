use std::path::Path;

use base64::{self, engine::general_purpose, Engine};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use leptos::*;
use regex::Regex;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{
    js_sys::{ArrayBuffer, Uint8Array},
    Event, FileReader, HtmlInputElement, MouseEvent, Url,
};

use crate::components::{select::*, slider::*, upload::*};

#[component]
pub fn Sidebar(setImage: WriteSignal<Option<DynamicImage>>) -> impl IntoView {
    let uploadRef: NodeRef<html::Input> = create_node_ref();

    let (imageUrl, setImageUrl) = create_signal::<Option<String>>(None);

    let displayImage = move || {
        imageUrl().map(|_| {
            // TODO: Add option to delete image and start over
            Some(view! {<img src=imageUrl alt="Unable to display uploaded image" class="w-full"/>})
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

            let (w, h) = img.dimensions();
            let nw = 1000;
            let nh = nw * h / w;
            // TODO: Plugin filter states to modify image. Reset filter states when a new image is uploaded
            let img = img
                // .adjust_contrast(16.1)
                // .brighten(50)
                .resize(nw, nh, FilterType::Nearest)
                .resize_exact(nw / 7, nh / 14, FilterType::Lanczos3);
            setImage(Some(img));
        }) as Box<dyn FnMut()>);

        reader
            .read_as_array_buffer(&file)
            .expect("Failed to load image into buffer");
        reader.set_onload(Some(onloadClosure.as_ref().unchecked_ref()));
        onloadClosure.forget();
    };

    let displayInput = move || {
        imageUrl().map_or_else(
            || Some(view! { <Upload onUpload=onImageUpload inputRef=uploadRef/>}),
            |_| None,
        )
    };

    let onContrastChange = move |e: Event| {
        let c: f32 = event_target_value(&e)
            .parse()
            .expect("Contrast should be a number");
        logging::log!("Adjusting contrast to {:?}...", &c);
        setImage.update(|img: &mut Option<DynamicImage>| {
            *img = img.as_ref().map(|img| img.adjust_contrast(c))
        });
    };

    let onBrightnessChange = move |e: Event| {
        let b: i32 = event_target_value(&e)
            .parse()
            .expect("Contrast should be a number");
        logging::log!("Adjusting brightness to {:?}...", &b);
        setImage.update(|img: &mut Option<DynamicImage>| {
            *img = img.as_ref().map(|img| img.brighten(b))
        });
    };

    let onSizeChange = |e: Event| logging::log!("New Size: {:?}", event_target_value(&e));

    let onDitherChange = |e: Event| logging::log!("New Dither: {:?}", event_target_value(&e));

    view! {
        <aside class="w-72 h-auto p-5 rounded-xl bg-neutral-50 drop-shadow-lg display-flex flex-col space-y-8 overflow-y-auto">
            {displayImage}
            {displayInput}
            <Select name="font".to_string() label="Font".to_string()/>
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
            <Slider
                name="dither".to_string()
                label="Dither".to_string()
                min=10
                max=300
                value=100
                onInput=onDitherChange
            />
        </aside>
    }
}

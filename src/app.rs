use image::DynamicImage;
use leptos::*;

use crate::sections::{artpanel::*, sidebar::*};

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
    let (image, set_image) = create_signal::<Option<DynamicImage>>(None);
    let grayImage = move || image().map(|img| img.into_luma8());

    view! {
        <div class="w-full h-full p-8 bg-amber-100 flex flex-row space-x-8">
            <Sidebar setImage=set_image/>
            <ArtPanel grayImage/>
        </div>
    }
}

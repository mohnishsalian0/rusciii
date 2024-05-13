#![allow(dead_code, unused, non_snake_case, clippy::new_without_default)]

pub mod app;
pub mod asciiGenerator;
pub mod components;
pub mod imageHandler;
pub mod sections;
pub mod utils;

use leptos::*;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

use leptos::*;
use web_sys::{Event, MouseEvent};

#[component]
pub fn Upload<F>(onUpload: F, inputRef: NodeRef<html::Input>) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    let onClick = move |e: MouseEvent| {
        inputRef().expect("Upload input should be mounted").click();
    };

    let onUploadClick = move |e: MouseEvent| {
        // Need to stop propagation to prevent event bubbling and avoid closure invoked recursively error
        e.stop_propagation();
    };

    view! {
        <button class="w-full relative aspect-video" on:click=onClick>
            <div class="absolute inset-0 p-8 border-b border-amber-500 flex flex-col justify-center">
                <h4 class="text-xl font-medium text-center text-zinc-800">Upload image</h4>
                <p class="text-sm font-normal text-center text-zinc-800">
                    Supported image: png, jpg, jpeg, Max size: 5MB
                </p>
            </div>
            <input
                type="file"
                id="img"
                name="img"
                accept="image/png, image/jpg, image/jpeg"
                class="hidden"
                // Apart from stop propagation the call also needs to be undelegated to avoid error
                on:click:undelegated=onUploadClick
                on:change=onUpload
                node_ref=inputRef
            />
        </button>
    }
}

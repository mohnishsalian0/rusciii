use ::leptos::*;
use web_sys::Event;

#[component]
pub fn Slider<F>(
    name: String,
    label: String,
    min: i16,
    max: i16,
    value: i16,
    onInput: F,
) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <fieldset>
            <label class="text-xl font-medium text-indigo-500" for=name.clone()>
                {label}
            </label>
            <input
                id=name.clone()
                class="w-full px-4 accent-amber-500"
                type="range"
                min=min
                max=max
                value=value
                on:input=onInput
            />
        </fieldset>
    }
}

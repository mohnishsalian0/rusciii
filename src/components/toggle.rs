use leptos::*;
use web_sys::Event;

#[component]
pub fn Toggle<F>(name: String, label: String, onInput: F) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <fieldset>
            <label class="w-full inline-flex justify-between items-center cursor-pointer">
            <span class="text-xl font-medium text-zinc-800">{label}</span>
            <input type="checkbox" value="" class="sr-only peer" on:change=onInput/>
            <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-cyan-500 dark:bg-slate-200 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-cyan-600"></div>
            </label>
        </fieldset>
    }
}

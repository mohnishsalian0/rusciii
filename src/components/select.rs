use leptos::*;

#[component]
pub fn Select(name: String, label: String) -> impl IntoView {
    view! {
        <fieldset>
            <label class="text-xl font-medium text-zinc-800" for=name.clone()>
                {label}
            </label>
            <select
                class="w-full p-2 rounded-sm bg-transparent border border-amber-500 text-sm"
                name=name.clone()
                id=name
            >
                <option value="menlo">Menlo</option>
                <option value="courierNew">Courier New</option>
            </select>
        </fieldset>
    }
}

use ::leptos::*;

#[component]
pub fn Select(name: String, label: String) -> impl IntoView {
    view! {
        <fieldset>
            <label class="text-neutral-500" for=name.clone()>
                {label}
            </label>
            <select
                class="w-full p-2 rounded-sm bg-neutral-50 border border-neutral-200 text-slate-900"
                name=name.clone()
                id=name
            >
                <option value="menlo">Menlo</option>
                <option value="courierNew">Courier New</option>
            </select>
        </fieldset>
    }
}

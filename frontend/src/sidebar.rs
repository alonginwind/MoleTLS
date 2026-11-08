use crate::i18n::I18n;
use leptos::*;

#[component]
pub fn Sidebar(page: RwSignal<&'static str>) -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    const MENU_KEYS: [&'static str; 3] = ["status", "services", "settings"];

    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                "🐭 MoleTLS"
            </div>
            {MENU_KEYS.iter().map(|key| {
                let page_key = *key;
                let is_active = move || page.get() == page_key;
                view! {
                    <div
                        on:click=move |_| page.set(page_key)
                        class=move || if is_active() { "menu-item active" } else { "menu-item" }
                    >
                        {move || i18n.t(key)}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

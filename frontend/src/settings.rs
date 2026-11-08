use crate::i18n::{I18n, Locale};
use leptos::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    view! {
        <div class="card">
            <h2 class="card-title">{move || i18n.t("settings")}</h2>
            <div class="setting-group">
                <div class="setting-label">{move || i18n.t("language")}</div>
                <div class="lang-buttons">
                    {Locale::all().iter().map(|loc| {
                        let loc = *loc;
                        let is_current = move || i18n.locale.get() == loc;
                        view! {
                            <button
                                on:click=move |_| i18n.locale.set(loc)
                                class=move || if is_current() { "lang-btn active" } else { "lang-btn" }
                            >
                                {loc.name()}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

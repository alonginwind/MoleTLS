use crate::i18n::I18n;
use leptos::*;

#[component]
pub fn ServicesPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    view! {
        <div class="card">
            <h2 class="card-title">{move || i18n.t("services")}</h2>
            <div class="empty-state">
                {move || i18n.t("no_services")}
            </div>
        </div>
    }
}

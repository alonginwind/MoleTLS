mod i18n;
mod lang;
mod services;
mod settings;
mod sidebar;
mod status;

use i18n::{I18n, Locale};
use leptos::*;
use services::ServicesPage;
use settings::SettingsPage;
use sidebar::Sidebar;
use status::StatusPage;

#[component]
fn App() -> impl IntoView {
    let page = create_rw_signal("status");
    let i18n = I18n::new(Locale::Zh);
    provide_context(i18n);

    view! {
        <div class="app-layout">
            <Sidebar page=page />
            <div class="content">
                {move || match page.get() {
                    "status" => view! { <StatusPage /> }.into_view(),
                    "services" => view! { <ServicesPage /> }.into_view(),
                    "settings" => view! { <SettingsPage /> }.into_view(),
                    _ => view! {}.into_view(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}

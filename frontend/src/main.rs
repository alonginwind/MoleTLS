mod i18n;
mod lang;

use gloo_net::http::Request;
use i18n::{I18n, Locale};
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Status {
    status: String,
    backend: String,
    port: u16,
}

#[component]
fn Sidebar(page: RwSignal<&'static str>) -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    const MENU_KEYS: [&'static str; 3] = ["status", "services", "settings"];

    view! {
        <div style="width: 200px; background: white; border-right: 1px solid #e8e8e8; padding: 20px 0; min-height: 100vh;">
            <div style="padding: 0 20px 20px; font-size: 18px; font-weight: bold; border-bottom: 1px solid #e8e8e8; margin-bottom: 10px; color: #333;">
                "🐭 MoleTLS"
            </div>
            {MENU_KEYS.iter().map(|key| {
                let page_key = *key;
                let is_active = move || page.get() == page_key;
                view! {
                    <div
                        on:click=move |_| page.set(page_key)
                        style=move || {
                            let bg = if is_active() { "#f0f7ff" } else { "transparent" };
                            let color = if is_active() { "#1890ff" } else { "#555" };
                            format!("padding: 12px 20px; cursor: pointer; background: {}; color: {}; border-left: 3px solid {}", bg, color, if is_active() { "#1890ff" } else { "transparent" })
                        }
                    >
                        {move || i18n.t(key)}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn StatusPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");
    let status = create_rw_signal::<Option<Status>>(None);

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(s) = Request::get("/api/status").send().await {
                if let Ok(s) = s.json::<Status>().await {
                    status.set(Some(s));
                }
            }
        });
    });

    view! {
        <div>
            <div style="background: white; padding: 25px; border-radius: 8px; border: 1px solid #e0e0e0;">
                <h2 style="margin: 0 0 20px 0; font-size: 20px;">{move || i18n.t("system_status")}</h2>
                {move || match status.get() {
                    Some(s) => view! {
                        <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px;">
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">{move || i18n.t("status_label")}</div>
                                <div style="font-size: 16px; font-weight: 500;">
                                    <span style="color: #4CAF50;">"● "</span>
                                    {s.status}
                                </div>
                            </div>
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">{move || i18n.t("backend")}</div>
                                <div style="font-size: 16px; font-weight: 500;">{s.backend}</div>
                            </div>
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">{move || i18n.t("port")}</div>
                                <div style="font-size: 16px; font-weight: 500;">{s.port}</div>
                            </div>
                        </div>
                    }.into_view(),
                    None => view! { <p>{move || i18n.t("loading")}</p> }.into_view(),
                }}
            </div>
        </div>
    }
}

#[component]
fn ServicesPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    view! {
        <div style="background: white; padding: 25px; border-radius: 8px; border: 1px solid #e0e0e0;">
            <h2 style="margin: 0 0 20px 0; font-size: 20px;">{move || i18n.t("services")}</h2>
            <div style="padding: 60px 20px; text-align: center; color: #999; background: #f9f9f9; border-radius: 6px;">
                {move || i18n.t("no_services")}
            </div>
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");

    view! {
        <div style="background: white; padding: 25px; border-radius: 8px; border: 1px solid #e0e0e0;">
            <h2 style="margin: 0 0 20px 0; font-size: 20px;">{move || i18n.t("settings")}</h2>
            <div style="margin-bottom: 16px;">
                <div style="color: #666; font-size: 13px; margin-bottom: 8px;">{move || i18n.t("language")}</div>
                <div style="display: flex; gap: 8px;">
                    {Locale::all().iter().map(|loc| {
                        let loc = *loc;
                        let is_current = move || i18n.locale.get() == loc;
                        view! {
                            <button
                                on:click=move |_| i18n.locale.set(loc)
                                style=move || {
                                    let bg = if is_current() { "#1890ff" } else { "white" };
                                    let color = if is_current() { "white" } else { "#333" };
                                    let border = if is_current() { "1px solid #1890ff" } else { "1px solid #d9d9d9" };
                                    format!("padding: 6px 16px; cursor: pointer; border-radius: 4px; background: {}; color: {}; border: {}; font-size: 13px;", bg, color, border)
                                }
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

#[component]
fn App() -> impl IntoView {
    let page = create_rw_signal("status");
    let i18n = I18n::new(Locale::Zh);
    provide_context(i18n);

    view! {
        <div style="display: flex; min-height: 100vh; background: #f5f5f5;">
            <Sidebar page=page />
            <div style="flex: 1; padding: 30px;">
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

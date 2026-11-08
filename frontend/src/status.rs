use crate::i18n::I18n;
use gloo_net::http::Request;
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Status {
    status: String,
    backend: String,
    port: u16,
}

#[component]
pub fn StatusPage() -> impl IntoView {
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
            <div class="card">
                <h2 class="card-title">{move || i18n.t("system_status")}</h2>
                {move || match status.get() {
                    Some(s) => view! {
                        <div class="status-grid">
                            <div class="stat-box">
                                <div class="stat-label">{move || i18n.t("status_label")}</div>
                                <div class="stat-value"><span class="status-dot">"● "</span>{s.status}</div>
                            </div>
                            <div class="stat-box">
                                <div class="stat-label">{move || i18n.t("backend")}</div>
                                <div class="stat-value">{s.backend}</div>
                            </div>
                            <div class="stat-box">
                                <div class="stat-label">{move || i18n.t("port")}</div>
                                <div class="stat-value">{s.port}</div>
                            </div>
                        </div>
                    }.into_view(),
                    None => view! { <p>{move || i18n.t("loading")}</p> }.into_view(),
                }}
            </div>
        </div>
    }
}

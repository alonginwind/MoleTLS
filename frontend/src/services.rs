use crate::i18n::I18n;
use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Proxy {
    name: String,
    domain: String,
    target_url: String,
}

#[component]
pub fn ServicesPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");
    let services = create_rw_signal::<Vec<Proxy>>(Vec::new());
    let name = create_rw_signal(String::new());
    let domain = create_rw_signal(String::new());
    let target_url = create_rw_signal(String::new());

    // Fetch services on mount
    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(resp) = Request::get("/api/services").send().await {
                if let Ok(list) = resp.json::<Vec<Proxy>>().await {
                    services.set(list);
                }
            }
        });
    });

    let add_service = {
        let services = services;
        let name = name;
        let domain = domain;
        let target_url = target_url;
        move |_| {
            let proxy = Proxy {
                name: name.get(),
                domain: domain.get(),
                target_url: target_url.get(),
            };
            if proxy.name.is_empty() || proxy.domain.is_empty() || proxy.target_url.is_empty() {
                return;
            }
            spawn_local(async move {
                if let Ok(resp) = Request::post("/api/services")
                    .json(&proxy)
                    .unwrap()
                    .send()
                    .await
                {
                    if resp.ok() {
                        if let Ok(new_proxy) = resp.json::<Proxy>().await {
                            let mut list = services.get();
                            list.push(new_proxy);
                            services.set(list);
                        }
                    }
                }
                name.set(String::new());
                domain.set(String::new());
                target_url.set(String::new());
            });
        }
    };

    let delete_service = move |proxy_name: String| {
        let services = services;
        let name = proxy_name.clone();
        spawn_local(async move {
            if let Ok(resp) = Request::delete(&format!("/api/services/{}", name))
                .send()
                .await
            {
                if resp.ok() {
                    let mut list = services.get();
                    list.retain(|p| p.name != name);
                    services.set(list);
                }
            }
        });
    };

    view! {
        <div>
            <div class="card">
                <h2 class="card-title">{move || i18n.t("add_proxy")}</h2>
                <div class="proxy-form">
                    <div class="form-group name">
                        <label class="form-label">{move || i18n.t("name_label")}</label>
                        <input
                            class="form-input"
                            placeholder=move || i18n.t("name_placeholder")
                            prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group path">
                        <label class="form-label">{move || i18n.t("domain_label")}</label>
                        <input
                            class="form-input"
                            placeholder=move || i18n.t("domain_placeholder")
                            prop:value=move || domain.get()
                            on:input=move |ev| domain.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group url">
                        <label class="form-label">{move || i18n.t("target_url_label")}</label>
                        <input
                            class="form-input"
                            placeholder=move || i18n.t("target_placeholder")
                            prop:value=move || target_url.get()
                            on:input=move |ev| target_url.set(event_target_value(&ev))
                        />
                    </div>
                    <button class="btn-primary" on:click=add_service>
                        {move || i18n.t("add_btn")}
                    </button>
                </div>
            </div>

            <div class="card" style="margin-top: 20px;">
                <h2 class="card-title">{move || i18n.t("services")}</h2>
                {move || {
                    let list = services.get();
                    if list.is_empty() {
                        view! { <div class="empty-state">{move || i18n.t("no_services")}</div> }.into_view()
                    } else {
                        view! {
                            <div class="proxy-list">
                                {list.into_iter().map(|p| {
                                    let del_name = p.name.clone();
                                    let delete_handler = {
                                        let delete_service = delete_service;
                                        move |_| delete_service(del_name.clone())
                                    };
                                    view! {
                                        <div class="proxy-item">
                                            <span class="proxy-name">{p.name.clone()}</span>
                                            <span class="proxy-route">{p.domain.clone()}</span>
                                            <span class="proxy-target">"→ "{p.target_url.clone()}</span>
                                            <button class="btn-delete" on:click=delete_handler>"✕"</button>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

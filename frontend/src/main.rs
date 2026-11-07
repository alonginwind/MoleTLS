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
fn Sidebar(page: RwSignal<&'static str>) -> impl IntoView {
    const MENU_ITEMS: [(&'static str, &'static str); 2] = [
        ("status", "状态信息"),
        ("services", "网络服务"),
    ];

    view! {
        <div style="width: 200px; background: white; border-right: 1px solid #e8e8e8; padding: 20px 0; min-height: 100vh;">
            <div style="padding: 0 20px 20px; font-size: 18px; font-weight: bold; border-bottom: 1px solid #e8e8e8; margin-bottom: 10px; color: #333;">
                "🐭 MoleTLS"
            </div>
            {MENU_ITEMS.iter().map(|(key, label)| {
                let is_active = move || page.get() == *key;
                let page_key = *key;
                let label_text = *label;
                view! {
                    <div
                        on:click=move |_| page.set(page_key)
                        style=move || {
                            let bg = if is_active() { "#f0f7ff" } else { "transparent" };
                            let color = if is_active() { "#1890ff" } else { "#555" };
                            format!("padding: 12px 20px; cursor: pointer; background: {}; color: {}; border-left: 3px solid {}", bg, color, if is_active() { "#1890ff" } else { "transparent" })
                        }
                    >
                        {label_text}
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn StatusPage() -> impl IntoView {
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
                <h2 style="margin: 0 0 20px 0; font-size: 20px;">"系统状态"</h2>
                {move || match status.get() {
                    Some(s) => view! {
                        <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px;">
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">"状态"</div>
                                <div style="font-size: 16px; font-weight: 500;">
                                    <span style="color: #4CAF50;">"● "</span>
                                    {s.status}
                                </div>
                            </div>
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">"后端"</div>
                                <div style="font-size: 16px; font-weight: 500;">{s.backend}</div>
                            </div>
                            <div style="padding: 15px; background: #f9f9f9; border-radius: 6px;">
                                <div style="color: #666; font-size: 12px; margin-bottom: 5px;">"端口"</div>
                                <div style="font-size: 16px; font-weight: 500;">{s.port}</div>
                            </div>
                        </div>
                    }.into_view(),
                    None => view! { <p>"Loading..."</p> }.into_view(),
                }}
            </div>
        </div>
    }
}

#[component]
fn ServicesPage() -> impl IntoView {
    view! {
        <div style="background: white; padding: 25px; border-radius: 8px; border: 1px solid #e0e0e0;">
            <h2 style="margin: 0 0 20px 0; font-size: 20px;">"网络服务"</h2>
            <div style="padding: 60px 20px; text-align: center; color: #999; background: #f9f9f9; border-radius: 6px;">
                "暂无配置的服务"
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let page = create_rw_signal("status");

    view! {
        <div style="display: flex; min-height: 100vh; background: #f5f5f5;">
            <Sidebar page=page />
            <div style="flex: 1; padding: 30px;">
                {move || match page.get() {
                    "status" => view! { <StatusPage /> }.into_view(),
                    "services" => view! { <ServicesPage /> }.into_view(),
                    _ => view! {}.into_view(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}

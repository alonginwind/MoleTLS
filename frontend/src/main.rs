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
fn App() -> impl IntoView {
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
        <h1>"🐭 MoleTLS"</h1>
        {move || match status.get() {
            Some(s) => view! {
                <p>"Status: "{s.status}</p>
                <p>"Backend: "{s.backend}</p>
                <p>"Port: "{s.port}</p>
            }.into_view(),
            None => view! { <p>"Loading..."</p> }.into_view(),
        }}
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}

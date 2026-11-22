use crate::i18n::{I18n, Locale};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use gloo_net::http::{Request, RequestBuilder};
use leptos::*;

/// Admin session: holds the `Authorization` header value in memory only.
#[derive(Clone, Copy)]
pub struct Auth {
    header: RwSignal<Option<String>>,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            header: create_rw_signal(None),
        }
    }

    /// Builds a Basic auth header from plain credentials.
    pub fn basic(user: &str, pass: &str) -> String {
        format!("Basic {}", STANDARD.encode(format!("{user}:{pass}")))
    }

    pub fn is_logged_in(&self) -> bool {
        self.header.get().is_some()
    }

    pub fn sign_in(&self, header: String) {
        self.header.set(Some(header));
    }

    pub fn sign_out(&self) {
        self.header.set(None);
    }

    /// Attaches the credentials to an admin API request.
    pub fn sign(&self, rb: RequestBuilder) -> RequestBuilder {
        match self.header.get() {
            Some(header) => rb.header("authorization", &header),
            None => rb,
        }
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let i18n = use_context::<I18n>().expect("i18n context");
    let auth = use_context::<Auth>().expect("auth context");
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let failed = create_rw_signal(false);
    let busy = create_rw_signal(false);

    view! {
        <div class="login-page">
            <div class="card login-card">
                <h2 class="card-title">{move || i18n.t("login_title")}</h2>
                <form
                    class="login-form"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let header = Auth::basic(&username.get(), &password.get());
                        busy.set(true);
                        failed.set(false);
                        spawn_local(async move {
                            // No dedicated login endpoint: the credentials are validated
                            // by calling a protected API and checking the status code.
                            let ok = Request::get("/api/services")
                                .header("authorization", &header)
                                .send()
                                .await
                                .map(|resp| resp.ok())
                                .unwrap_or(false);
                            busy.set(false);
                            if ok {
                                auth.sign_in(header);
                            } else {
                                failed.set(true);
                            }
                        });
                    }
                >
                    <div class="form-group">
                        <label class="form-label">{move || i18n.t("username")}</label>
                        <input
                            class="form-input"
                            placeholder=move || i18n.t("username_placeholder")
                            prop:value=move || username.get()
                            on:input=move |ev| username.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="form-group">
                        <label class="form-label">{move || i18n.t("password")}</label>
                        <input
                            type="password"
                            class="form-input"
                            placeholder=move || i18n.t("password_placeholder")
                            prop:value=move || password.get()
                            on:input=move |ev| {
                                password.set(event_target_value(&ev));
                                failed.set(false);
                            }
                        />
                    </div>
                    {move || if failed.get() {
                        view! { <div class="login-error">{i18n.t("login_failed")}</div> }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                    <button class="btn-primary" type="submit" prop:disabled=move || busy.get()>
                        {move || i18n.t("login_btn")}
                    </button>
                </form>
                <div class="lang-buttons" style="margin-top: 20px;">
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

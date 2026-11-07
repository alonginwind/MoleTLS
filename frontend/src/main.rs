use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"🐭 MoleTLS"</h1>
        <p>"Status: Running"</p>
        <p>"Backend: httpbin.org"</p>
        <p>"Port: 7999"</p>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}

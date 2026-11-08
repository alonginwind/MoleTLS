pub fn t(key: &str) -> &'static str {
    match key {
        "status" => "Status",
        "services" => "Services",
        "system_status" => "System Status",
        "status_label" => "Status",
        "backend" => "Backend",
        "port" => "Port",
        "loading" => "Loading...",
        "no_services" => "No services configured",
        "settings" => "Settings",
        "language" => "Language",
        "add_proxy" => "Add Proxy",
        "name_label" => "Name",
        "domain_label" => "Domain",
        "target_url_label" => "Target URL",
        "name_placeholder" => "e.g. my-service",
        "domain_placeholder" => "e.g. kk.test.cn",
        "target_placeholder" => "e.g. http://localhost:8080",
        "add_btn" => "Add",
        _ => "??",
    }
}

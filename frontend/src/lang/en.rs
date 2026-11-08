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
        _ => "??",
    }
}

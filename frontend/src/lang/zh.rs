pub fn t(key: &str) -> &'static str {
    match key {
        "status" => "状态信息",
        "services" => "网络服务",
        "system_status" => "系统状态",
        "status_label" => "状态",
        "backend" => "后端",
        "port" => "端口",
        "loading" => "加载中...",
        "no_services" => "暂无配置的服务",
        "settings" => "系统设置",
        "language" => "语言",
        _ => "??",
    }
}

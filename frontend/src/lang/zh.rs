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
        "add_proxy" => "添加反代",
        "name_label" => "名称",
        "domain_label" => "域名",
        "target_url_label" => "目标地址",
        "name_placeholder" => "例如: my-service",
        "domain_placeholder" => "例如: kk.test.cn",
        "target_placeholder" => "例如: http://localhost:8080",
        "add_btn" => "添加",
        _ => "??",
    }
}

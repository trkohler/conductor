use conductor::run_services;

#[tokio::main]
async fn main() {
    let config_file_path = std::env::args()
        .nth(1)
        .unwrap_or("./config.json".to_string());

    run_services(config_file_path).await;
}

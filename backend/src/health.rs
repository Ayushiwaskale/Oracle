use tracing::info;

pub async fn start_health_loop() {
    tokio::spawn(async move {
        loop {
            info!("health tick");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
}

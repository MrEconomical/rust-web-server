mod r#async;

#[tokio::main]
async fn main() {
    r#async::main().await;
}
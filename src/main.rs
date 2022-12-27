use exchange_api::alpaca;

#[tokio::main]
async fn main() {
  alpaca::buy("AAPL", 1, 100).await;
}
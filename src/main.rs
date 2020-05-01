mod stocks;

use stocks::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stocks_repo = StockRepo::new();
    stocks_repo.get_stock_price("T").await.unwrap();
    Ok(())
}



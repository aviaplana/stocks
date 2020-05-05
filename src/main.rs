mod repository;

use repository::{
    StockRepository
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repository = StockRepository::new().unwrap();

    if let Ok(stocks) = repository.get_available_stocks().await {
        for stock in stocks {
            println!("{:?}", stock);
        }
    }
    
    Ok(())
}



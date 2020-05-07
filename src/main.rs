mod repository;
mod server;

use repository::{
    StockRepository,
    stock::Stock,
};
use tokio;

#[derive(Debug)]
pub enum Operation {
    ListStored,
    ListAvailable,
    UpdatePrices,
    Delete(String),
    Error
}

impl From<String> for Operation {
    fn from(val: String) -> Operation {
        match val.as_str() {
            "list_stored" => Self::ListStored,
            "list_available" => Self::ListAvailable,
            "update_prices" => Self::UpdatePrices,
            op if op.starts_with("delete") => {
                let parts = op.split(' ').collect::<Vec<&str>>();
                let stock = parts.get(1).unwrap();
                Self::Delete(String::from(*stock))
            },
            _ => Self::Error
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repository = StockRepository::new().unwrap();

    let rx_ch = server::launch_tcp_server();

    loop {
        let operation = Operation::from(rx_ch.recv().unwrap());
        println!("Channel got : {:?}", operation);
        
        match operation {
            Operation::ListAvailable => {
                
            },
            _ => {}
        }

    }
    Ok(())
}





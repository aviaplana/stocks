mod repository;
mod server;

use hyper_tls::HttpsConnector;
use tokio;
use repository::{
    HttpClient
};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2;

#[derive(Debug, PartialEq)]
pub enum Operation {
    ListStored,
    ListAvailable,
    UpdatePrices,
    DeleteStock(String),
    Help,
    Error
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            ListStored => "list_stored".into(),
            ListAvailable => "list_available".into(),
            UpdatePrices => "update_prices".into(),
            Delete => "delete_stock".into(),
            Help => "help".into(),
            Error => "error".into(),
        }
    }
}

impl From<String> for Operation {
    fn from(val: String) -> Operation {
        match val.as_str() {
            "list_stored" => Self::ListStored,
            "list_available" => Self::ListAvailable,
            "update_prices" => Self::UpdatePrices,
            op if op.starts_with("delete_stock") => {
                let parts = op.split(' ').collect::<Vec<&str>>();
                let stock = parts.get(1).unwrap();
                Self::DeleteStock(String::from(*stock))
            },
            "help" | "?" => Self::Help,
            _ => Self::Error
        }
    }
}

fn get_db_pool_connection() -> r2d2::Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("stocks.db");
    r2d2::Pool::new(manager).expect("Couldn't create pool.")
}

fn get_hyper_connection() -> HttpClient{
    let https = HttpsConnector::new();
    hyper::Client::builder().build::<_, hyper::Body>(https)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_pool = get_db_pool_connection();
    let rx_ch = server::launch_tcp_server();

    loop {
        let mut received_str = rx_ch.recv().unwrap();
        received_str.pop(); // Pop removes tailing new line.
        let operation = Operation::from(received_str);

        println!("Channel got : {:?}", operation);
        let pool = db_pool.clone();
        tokio::task::spawn(async move {
            match operation {
                Operation::ListStored => {
                    let connection = pool.get().unwrap();
                    repository::get_stored_stocks(&connection);
                },
                Operation::ListAvailable => {
                    let http_client = get_hyper_connection();
                    for stock in repository::get_available_stocks(&http_client).await {
                        println!("{:?}", stock);
                    };
                },
                Operation::Help => {
                    println!(r"Available commands:{}, {}, {}",
                        Operation::ListAvailable.to_string(), 
                        Operation::ListStored.to_string(),
                        Operation::Help.to_string());
                }
                _ => {}
            }
        }).await.unwrap()
    }
    Ok(())
}

#[test]
fn test_str_to_operation_list_available() {
    let raw = "list_available".to_owned();
    let operation = Operation::from(raw);
    assert_eq!(Operation::ListAvailable, operation);
}

#[test]
fn test_str_to_operation_error() {
    let raw = "fail".to_owned();
    let operation = Operation::from(raw);
    assert_eq!(Operation::Error, operation);
}
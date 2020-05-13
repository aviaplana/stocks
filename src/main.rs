mod repository;
mod server;

use log::{debug, info};
use crossbeam_channel::Sender;
use serde::{Serialize, Deserialize};
use hyper_tls::HttpsConnector;
use tokio;
use server::ResponseWrapper;
use repository::{
    stock::Stock,
    HttpClient
};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    ListStored,
    ListAvailable,
    UpdatePrices,
    DeleteStock(String),
    AddStock(Stock),
    Help,
    Error
}

pub trait ToBytes  {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T> ToBytes for T where 
    T: Serialize + Deserialize<'static>{
    
        fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::ListStored => "list_stored".into(),
            Self::ListAvailable => "list_available".into(),
            Self::UpdatePrices => "update_prices".into(),
            Self::AddStock(_) => "add_stock".into(),
            Self::DeleteStock(_) => "delete_stock".into(),
            Self::Help => "help".into(),
            Self::Error => "error".into(),
        }
    }
}

impl From<&Vec<u8>> for Operation {
    fn from(vec: &Vec<u8>) -> Operation {
        bincode::deserialize(vec.as_slice()).unwrap()
    }
}

impl From<String> for Operation {
    fn from(val: String) -> Operation {
        match val.as_str() {
            "list_stored" => Self::ListStored,
            "list_available" => Self::ListAvailable,
            "update_prices" => Self::UpdatePrices,
            op if op.starts_with("delete_stock") => {
                let parts = op.split_whitespace().collect::<Vec<&str>>();
                let stock = parts.get(1).unwrap();
                Self::DeleteStock(String::from(*stock))
            },
            op if op.starts_with("add_stock") => {
                let parts = op.split_whitespace().collect::<Vec<&str>>();
                let stock_str = parts[1..].join(" ");
                let stock = Stock::from(stock_str.as_str());
                Self::AddStock(stock)
            },
            "help" | "?" => Self::Help,
            _ => Self::Error
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    id: u32,
    payload: Vec<u8>,
}

impl From<&Vec<u8>> for Job {
    fn from(vec: &Vec<u8>) -> Job {
        bincode::deserialize(vec.as_slice()).unwrap()
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
    env_logger::init();
    let db_pool = get_db_pool_connection();
    let rx_ch = server::launch_tcp_server();

    loop {
        info!(target: "Main", "Waiting for messages...");
        
        let (received_str, tx_ch) = rx_ch.recv().unwrap();
        debug!(target: "Main", "Reived raw: {:?}", &received_str);

        let job = Job::from(&received_str);
        debug!(target: "Main", "Deserialized{:?}", job);

        let operation = Operation::from(&job.payload);
        info!(target: "Main", "Got operation {} from connection {}", operation.to_string(), job.id);

        let pool = db_pool.clone();

        tokio::task::spawn(async move {
            match operation {
                Operation::ListStored => process_list_stored(tx_ch, job.id, &pool),
                Operation::ListAvailable => process_list_available(tx_ch, job.id).await,
                Operation::AddStock(stock) => process_add_stock(tx_ch, job.id, &stock, &pool),
                Operation::DeleteStock(symbol) => process_delete_stock(tx_ch, job.id, &symbol, &pool),
                Operation::Help => process_help(tx_ch, job.id),
                _ => {}
            }
        }).await.unwrap()
    }

    Ok(())
}

async fn process_list_available(tx: Sender<Vec<u8>>, id: u32) {
    let http_client = get_hyper_connection();
    let list_available = repository::get_available_stocks(&http_client).await.unwrap();
    let list_json = serde_json::to_string(&list_available).unwrap();


    send_response(&tx, id, &list_json);
}

fn process_add_stock(tx: Sender<Vec<u8>>, id: u32, stock: &Stock, pool: &r2d2::Pool<SqliteConnectionManager>) {
    let connection = pool.get().unwrap();
    let response = repository::add_stock(&connection, stock)
        .map(|_| { "true"})
        .or_else::<&str, _>(|_| { Ok("false") })
        .unwrap();
    send_response(&tx, id, response);
}

fn process_delete_stock(tx: Sender<Vec<u8>>, id: u32, stock: &str, pool: &r2d2::Pool<SqliteConnectionManager>) {
    let connection = pool.get().unwrap();
    let response = repository::delete_stock(&connection, stock)
        .map(|_| { "true" })
        .or_else::<&str, _>(|_| { Ok("false") })
        .unwrap();
    send_response(&tx, id, response.into());
}

fn process_list_stored(tx: Sender<Vec<u8>>, id: u32, pool: &r2d2::Pool<SqliteConnectionManager>) {
    let connection = pool.get().unwrap();
    let response = repository::get_stored_stocks(&connection).unwrap();

    let serialized_response = serde_json::to_string(&response).unwrap();
    send_response(&tx, id, serialized_response.as_str());
}

fn process_help(tx: Sender<Vec<u8>>, id: u32) {
    let response = format!("Available commands:{}, {}, {}",
        Operation::ListAvailable.to_string(), 
        Operation::ListStored.to_string(),
        Operation::Help.to_string());
        
    send_response(&tx, id, response.as_str());
}

fn send_response(tx: &Sender<Vec<u8>>, id: u32, response: &str) {
    let response = ResponseWrapper{ response };
    let response_serialized = serde_json::to_string(&response).unwrap();
    
    info!(target: "Main", "Sending to connection {}: {}", id, &response_serialized);
    
    let job_response = Job {
        id,
        payload: response_serialized.into_bytes()
    };
    
    let bytes = job_response.to_bytes();
    tx.send(bytes).unwrap();
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

extern crate hyper;
use hyper::{
    Client, 
    Body,
    body::to_bytes,
    Uri,
    client::HttpConnector,
};
use hyper_tls::HttpsConnector;
use serde_json;
use serde::Deserialize;

const BASE_URL: &'static str = "https://financialmodelingprep.com/";

enum Endpoint {
    RealTimePrice(String),
    StockList,
}

impl Endpoint {
    pub fn to_uri(&self) -> Uri {
        let route = match self {
            Self::RealTimePrice(args) => "api/v3/stock/real-time-price/".to_owned() + args,
            Self::StockList => "api/v3/company/stock/list".into()
        };

        let base = String::from(BASE_URL);

        format!("{}{}", base, route).parse().unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    pub symbol: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub price: f32,
}

#[derive(Deserialize)]
struct StocksListResponse {
    symbolsList: Vec<StockPrice>,
}

pub struct StockApi { 
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl StockApi {
    pub fn new() -> StockApi {
        let https = HttpsConnector::new();
        StockApi {
            client: Client::builder().build::<_, hyper::Body>(https)
        }
    }

    pub async fn get_stock_list(&self) -> Result<Vec<StockPrice>, Box<dyn std::error::Error + Send + Sync>> {        
        let uri = Endpoint::StockList.to_uri();
        let response = self.client.get(uri).await;
        match response {
            Ok(mut resp) => {
                match to_bytes(resp.body_mut()).await {
                    Ok(body) => {
                        let stock_price: StocksListResponse = serde_json::from_slice(&body).unwrap();
                        //println!("{:?}", stock_price);
                        Ok(stock_price.symbolsList)
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_stock_price(&self, symbol: &str) -> Result<StockPrice, Box<dyn std::error::Error + Send + Sync>> {        
        let uri = Endpoint::RealTimePrice(symbol.into()).to_uri();
        let response = self.client.get(uri).await;

        match response {
            Ok(mut resp) => {
                match to_bytes(resp.body_mut()).await {
                    Ok(body) => {
                        let stock_price: StockPrice = serde_json::from_slice(&body).unwrap();
                        println!("{:?}", stock_price);
                        Ok(stock_price)
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into()),
        }
    }
}



/*



https://financialmodelingprep.com/api/v3/company/stock/list
{
  "symbolsList" : [ {
    "symbol" : "SPY",
    "name" : "SPDR S&P 500",
    "price" : 325.64
  }, {
    "symbol" : "CMCSA",
    "name" : "Comcast Corporation Class A Common Stock",
    "price" : 44.98
  }
} 
*/


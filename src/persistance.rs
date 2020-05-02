use std::collections::HashMap;

enum PersistanceError {
    KeyNotFoundError
}


#[derive(Debug, PartialEq, Clone)]
enum StockMarket {
    NASDAQ,
    DOWN_JONES,
    IBEX35,
    CRYPTO
}

#[derive(Debug, PartialEq, Clone)]
pub struct Asset {
    symbol: String,
    price: f32,
    initial_price: f32,
    market: StockMarket
}

impl Asset {
    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }
}

pub struct StockRepository {
    entries: HashMap<String, Asset>,
}

impl StockRepository {
    pub fn new() -> StockRepository {
        let mut entries = HashMap::new();

        let mock_asset = Asset {
            symbol: String::from("AAPL"),
            price: 123.4,
            initial_price: 12.3,
            market: StockMarket::DOWN_JONES
        };

        let mock_asset2 = Asset {
            symbol: String::from("BTC"),
            price: 8000.0,
            initial_price: 7990.0,
            market: StockMarket::CRYPTO
        };

        entries.insert(mock_asset.symbol.to_owned(), mock_asset);
        entries.insert(mock_asset2.symbol.to_owned(), mock_asset2);

        StockRepository {
            entries
        }
    }

    fn add(&mut self, entry: Asset) {
        self.entries.insert(entry.symbol.to_owned(), entry);
    }

    pub fn update(&mut self, entry: &Asset) -> Result<(), PersistanceError> {
        if self.entries.contains_key(&entry.symbol) {
            self.entries.insert(entry.symbol.to_owned(), entry.to_owned());
            Ok(())
        } else {
            Err(PersistanceError::KeyNotFoundError)
        }
    }

    pub fn delete(&mut self, symbol: &str) -> Result<(), PersistanceError> {
        if self.entries.contains_key(symbol) {
            self.entries.remove(symbol);
            Ok(())
        } else {
            Err(PersistanceError::KeyNotFoundError)
        }
    }

    pub fn get_entries(&self) -> Vec<&Asset> {
        self.entries.values().collect::<Vec<&Asset>>()
    }
}

#[test]
fn test_add() {
    let mut persistance = StockRepository::new();
    
    let mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());
    let entries = persistance.get_entries();

    assert!(entries.iter().any(|&x| x == &mock_asset));
}

#[test]
fn test_update() {
    let mut persistance = StockRepository::new();
    
    let mut mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());

    let new_price = 12332.3;
    mock_asset.set_price(new_price);
    assert!(persistance.update(&mock_asset).is_ok());

    let entries = persistance.get_entries();

    assert!(entries.iter()
        .find(|&&x| x.symbol == mock_asset.symbol)
        .unwrap()
        .price == new_price);
}


#[test]
fn test_delete() {
    let mut persistance = StockRepository::new();
    
    let mock_asset = get_asset_mock();

    persistance.add(mock_asset.clone());

    assert!(persistance.delete(&mock_asset.symbol).is_ok());
}


fn get_asset_mock() -> Asset {
    Asset {
        symbol: String::from("BTC"),
        price: 8000.0,
        initial_price: 7990.0,
        market: StockMarket::CRYPTO
    }
}
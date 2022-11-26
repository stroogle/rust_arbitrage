use serde_json;
use serde_json::{Value};
use std::f64;
use serde::{Deserialize};
use regex::{Regex};


#[derive(Deserialize)]
#[derive(Clone)]
pub struct Ticker {
    pub symbol: String,
    pub asking_price: f64,
    pub asking_qty: f64,
    pub bid_price: f64,
    pub bid_qty: f64,
    pub base_asset: String,
    pub quote_asset: String
}

impl Ticker {
    pub fn new(base_asset: &str, quote_asset: &str) -> Ticker {
        
        Ticker::default(base_asset, quote_asset)
    }

    pub fn parse_float(s: String) -> f64 {
        let mut y = s.clone();
        y.remove(0);
        y.remove(y.len() - 1);
        y.parse().unwrap()
    }

    pub fn parse_symbol_from_json(json: &str) -> String {
        let res: Value = serde_json::from_str(json).unwrap();
        let result = res["s"].to_string().clone();
        let reg = Regex::new("[^A-Za-z]").unwrap();
        // result.replace(reg, "")
        reg.replace_all(&result, "").to_string()
    }

    pub fn get_other_coin(&self, coin: String) -> String {
        let symbols = self.symbol.clone();
        let string_re = format!(r"(^{0})|({0}$)", coin);
        let re = Regex::new(&string_re).unwrap();
        let res = re.replace(&symbols, "").to_string();
        res
    }

    pub fn default(base_asset: &str, quote_asset: &str) -> Ticker {
        let symbol = format!("{}{}", base_asset, quote_asset);

        let asking_price: f64 = 1.0;
        let asking_qty: f64 = 1.0;
        let bid_price: f64 = 1.0;
        let bid_qty: f64 = 1.0;

        Ticker {
            symbol,
            asking_price,
            asking_qty,
            bid_price,
            bid_qty,
            base_asset: base_asset.to_string(),
            quote_asset: quote_asset.to_string(),
        }
    }

    pub fn update(&mut self, json: String) {
        let res: Value = serde_json::from_str(&json).unwrap();

        self.asking_price = Ticker::parse_float(res["a"].to_string());
        self.asking_qty= Ticker::parse_float(res["A"].to_string());
        self.bid_price= Ticker::parse_float(res["b"].to_string());
        self.bid_qty= Ticker::parse_float(res["B"].to_string());
    }
 }

 #[cfg(test)]
 mod tests {

    use super::*;

    #[test]
    pub fn test_get_other_coin() {
        let other_coin_1 = Ticker::new("ETH", "BUSD");

        assert_eq!(
            other_coin_1.get_other_coin("BUSD".to_string()),
            "ETH".to_string()
        );

        let other_coin_2 = Ticker::new("BUSD", "ETH");

        assert_eq!(
            other_coin_2.get_other_coin("BUSD".to_string()),
            "ETH".to_string()
        );
    }

    #[test]
    fn test_parse_symbol_from_json() {
        let sym = "BNBUSDT".to_string();
        let json_1 = "{
            \"u\":400900217,
            \"s\":\"BNBUSDT\",
            \"b\":\"25.35190000\",
            \"B\":\"31.21000000\",
            \"a\":\"25.36520000\",
            \"A\":\"40.66000000\"
          }";

        let result = Ticker::parse_symbol_from_json(json_1).clone();
        assert_eq!(result, sym.clone())
          
    }

 }
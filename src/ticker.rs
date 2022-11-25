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
    pub bid_qty: f64
}

impl Ticker {
    pub fn new(json: String) -> Result<Ticker, &'static str> {
        let res: Value = serde_json::from_str(&json).unwrap();

        let asking_price= Ticker::parse_float(res["a"].to_string());
        let asking_qty= Ticker::parse_float(res["A"].to_string());
        let bid_price= Ticker::parse_float(res["b"].to_string());
        let bid_qty= Ticker::parse_float(res["B"].to_string());
        // let base_quote = res[""]

        return Ok(Ticker {
            symbol: res["s"].to_string(),
            asking_price,
            asking_qty,
            bid_price,
            bid_qty,
        })
    }

    pub fn parse_float(s: String) -> f64 {
        let mut y = s.clone();
        y.remove(0);
        y.remove(y.len() - 1);
        y.parse().unwrap()
    }

    pub fn get_other_coin(&self, coin: String) -> String {
        let symbols = self.symbol.clone();
        let string_re = format!(r"(^{0})|({0}$)", coin);
        let re = Regex::new(&string_re).unwrap();
        let res = re.replace(&symbols, "").to_string();
        res
    }
 }

 #[cfg(test)]
 mod tests {

    use super::*;

    #[test]
    pub fn test_get_other_coin() {
        let other_coin_1 = Ticker {
            symbol: "ETHBUSD".to_string(),
            asking_price: 1.000,
            asking_qty: 1.000,
            bid_price: 1.000,
            bid_qty: 1.000
        };

        assert_eq!(
            other_coin_1.get_other_coin("BUSD".to_string()),
            "ETH".to_string()
        );

        let other_coin_2 = Ticker {
            symbol: "BUSDETH".to_string(),
            asking_price: 1.000,
            asking_qty: 1.000,
            bid_price: 1.000,
            bid_qty: 1.000
        };

        assert_eq!(
            other_coin_2.get_other_coin("BUSD".to_string()),
            "ETH".to_string()
        );
    }

 }
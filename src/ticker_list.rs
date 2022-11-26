use crate::Ticker;
use std::collections::HashMap;
use std::fs;

pub struct TickerList {

}

impl TickerList {
    pub fn get_ticker_hashmap(path: &str) -> HashMap<String, Ticker> {
        let mut map: HashMap<String, Ticker> = HashMap::new();

        let file_contents = TickerList::read_file(&path);

        for line in file_contents.lines() {
            let assets: Vec<&str> = line.split("|").collect();
            let ticker = Ticker::new(assets[0], assets[1]);
            map.insert(ticker.symbol.clone(), ticker);
        }

        return map;

    }

    fn read_file(path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }
}
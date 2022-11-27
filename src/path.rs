use std::fs;
use std::process;
use crate::Ticker;
use std::collections::HashMap;

pub struct Path {
    pub first_trade: String,
    pub second_trade: String,
    pub third_trade: String,
}

impl Path {
    pub fn new(tick1: String, tick2: String, tick3: String) -> Path{
        Path {
            first_trade: tick1,
            second_trade: tick2,
            third_trade: tick3
        }
    }

    pub fn to_string(& self) -> String {
        let s = format!("{} -> {} -> {}", self.first_trade, self.second_trade, self.third_trade);
        return s;
    }

    pub fn is_profitable(&self, map: &HashMap<String, Ticker>) -> bool {
        // Base amount
        // Calculate potential
        // return if potential is 0.4% more than base amount
        false
    }

    pub fn calculate_potential(&self, amount: f64) -> f64 {
        // Store base amount
        // Calulate first trasnaction
        // Calulate second trasnaction
        // Calulate third trasnaction
        // Return the result of the third transaction
        1.0
    }

    pub fn parse_from_line(line: String) -> Result<Path, String> {
        let trimmed = line.trim().to_string();
        let split = trimmed.split("-");
        let mut tick_vec: Vec<String> = Vec::new();

        for tick in split.into_iter() {
            tick_vec.push(tick.to_string().clone());
        };

        if tick_vec.len() < 3 {
            return Err("Not enough trade routes".to_string());
        }

        Ok(Path::new(
            tick_vec[0].clone(),
            tick_vec[1].clone(),
            tick_vec[2].clone()
        ))
    }

    pub fn load_paths(path: &str) -> Vec<Path> {
        let file = fs::read_to_string(path).unwrap_or_else(|_err| {
            process::exit(1);
        });

        let mut vec: Vec<Path> = Vec::new();

        for line in file.lines() {
            vec.push(Path::parse_from_line(line.to_string().clone()).unwrap())
        }

        return vec;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::time;

    #[test]
    fn test_load_paths() {
        let t = Path::load_paths("./trade.paths");
        // assert_eq!(t.len(), 44);
    }

    #[test]
    fn test_path_loop_speed() {
        let paths = Path::load_paths("./trade.paths");

        let start = time::Instant::now();
        for path in paths.iter() {
            println!("{}", path.to_string());
        }

        let duration = start.elapsed();

        println!("{:?}", duration);
    }

    #[test]
    fn test_to_string() {
        let path1 = Path::new("A".to_string(), "B".to_string(), "C".to_string());
        assert_eq!(path1.to_string(), "A -> B -> C".to_string());

        let path2 = Path::new("ABC".to_string(), "BCD".to_string(), "CDE".to_string());
        assert_eq!(path2.to_string(), "ABC -> BCD -> CDE".to_string());
    }

    #[test]
    fn test_is_profitable() {
        // Given Path
        let path_1 = Path::parse_from_line(
            "BNBBUSD-XMRBNB-XMRBUSD\n".to_string()
        ).unwrap();
        // Given tickers for each trading ticker.
        let mut tickers: HashMap<String, Ticker> = HashMap::new();
        tickers.insert(
            path_1.first_trade.clone(),
            Ticker::new("BNB", "BUSD")
            // Ticker {
            //     symbol: "BNBBUSD".to_string(),
            //     asking_price: 1.00000,
            //     asking_qty: 1.00000,
            //     bid_price: 1.00000,
            //     bid_qty: 1.00000
            // }
        );
        // True

        // Given Path
        // Given tickers
        // False
    }

    #[test]
    fn test_parse_from_line() {
        let s1: String = "BNBBUSD-XMRBNB-XMRBUSD\n".to_string();
        let s1_result = Path::parse_from_line(s1.clone()).unwrap();
        assert_eq!(s1_result.first_trade, "BNBBUSD".to_string());
        assert_eq!(s1_result.second_trade, "XMRBNB".to_string());
        assert_eq!(s1_result.third_trade, "XMRBUSD".to_string());
    }

}
#![feature(get_mut_unchecked)]

// Here modules
mod path;
mod binance;
mod ticker;
mod ticker_list;
mod money_safe_math;

// Uses here
use binance::Binance;
use url::Url;
use tungstenite::{Message};
use ticker::Ticker;
use std::process;
use std::time;
use std::sync::{Arc};
use std::thread;
use std::collections::HashMap;
use ticker_list::TickerList;
use path::Path;

fn main() {
    
    // let all_tickers: Arc<HashMap<String, Ticker>> = Arc::new(HashMap::new());
    let all_tickers: Arc<HashMap<String, Ticker>> = Arc::new(TickerList::get_ticker_hashmap("./trade.tickers"));
    let all_paths = Path::load_paths("./trade.paths");
    let mut second_thread = Arc::clone(&all_tickers);

    let mut socket = Binance::new(
        Url::parse("wss://testnet.binance.vision/ws").unwrap(),
        "bruh".to_string().clone(),
        Vec::new()
    );

    socket.connection.write_message(Message::Text(r#"{
        "method": "SUBSCRIBE",
        "params":
        [
            "bnbeth@bookTicker",
            "ethusdt@bookTicker",
            "etceth@bookTicker",
            "zeceth@bookTicker",
            "xrpeth@bookTicker",
            "bnbusdt@bookTicker",
            "xmreth@bookTicker",
            "adaeth@bookTicker",
            "ltceth@bookTicker",
            "ltcusdt@bookTicker",
            "ltcbnb@bookTicker",
            "adausdt@bookTicker",
            "adabnb@bookTicker",
            "xrpusdt@bookTicker",
            "xrpbnb@bookTicker",
            "etcusdt@bookTicker",
            "etcbnb@bookTicker",
            "xmrbnb@bookTicker",
            "xmrusdt@bookTicker",
            "zecbnb@bookTicker",
            "zecusdt@bookTicker",
            "maticbnb@bookTicker",
            "maticusdt@bookTicker",
            "algobnb@bookTicker",
            "algousdt@bookTicker",
            "bnbbusd@bookTicker",
            "busdusdt@bookTicker",
            "xrpbusd@bookTicker",
            "ethbusd@bookTicker",
            "ltcbusd@bookTicker",
            "etcbusd@bookTicker",
            "adabusd@bookTicker",
            "algobusd@bookTicker",
            "xmrbusd@bookTicker",
            "zecbusd@bookTicker",
            "solbnb@bookTicker",
            "solusdt@bookTicker",
            "solbusd@bookTicker",
            "maticbusd@bookTicker",
            "ethdai@bookTicker",
            "bnbdai@bookTicker",
            "usdtdai@bookTicker",
            "busddai@bookTicker",
            "cakebnb@bookTicker",
            "cakebusd@bookTicker",
            "cakeusdt@bookTicker",
            "soleth@bookTicker",
            "maticeth@bookTicker",
            "algoeth@bookTicker"
        ],
        "id": 1
    }"#.into())).expect("Uh oh!");

    

    thread::spawn(move || {
        loop {
            // println!("Other thread looping");
            let start = time::Instant::now();
            // for(key, value) in second_thread.iter() {
            //    println!("[OTHER THREAD] {} is trading at: {}", key, value.asking_price);
            // }

            for path in all_paths.iter() {
                // if path.is_profitable() {
                //     path.execute(init_value);
                // }
                println!("[OTHER THREAD] {}", path.to_string());
            }
            // println!("{}", start.elapsed().as_millis());
            // thread::sleep(time::Duration::from_millis(100));
        }
    });

    
    let mut main_thread = Arc::clone(&all_tickers);

    loop {

        let msg = socket.connection.read_message().expect("Error reading message");
        // let start = SystemTime::now();
        let msg_as_string: String = String::from(msg.into_text().unwrap_or_else(|_err| {
            eprintln!("Failed to get message as string");
            process::exit(1)
        }));
        if msg_as_string.contains("null") {
            continue;
        }
        


        // let tick = Ticker::new("Bruh", "other bruh");

        // println!("[MAIN THREAD] {} is at an asking price of {}", tick.symbol, tick.asking_price);
        unsafe {
            let t = Arc::get_mut_unchecked(&mut main_thread);
            // t.insert(tick.symbol.clone(), tick);
            println!("{}", &msg_as_string);
            let ticker = t.get_mut(&Ticker::parse_symbol_from_json(&msg_as_string)).unwrap();

            ticker.update(msg_as_string);

        }

    }

}
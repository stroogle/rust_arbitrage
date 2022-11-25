#![feature(get_mut_unchecked)]

// Here modules
mod path;
mod binance;
mod ticker;

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

fn main() {
    
    let all_tickers: Arc<HashMap<String, Ticker>> = Arc::new(HashMap::new());
    let mut second_thread = Arc::clone(&all_tickers);

    let mut socket = Binance::new(
        Url::parse("wss://stream.binance.com:9443/ws").unwrap(),
        "bruh".to_string().clone(),
        Vec::new()
    );

    socket.connection.write_message(Message::Text(r#"{
        "method": "SUBSCRIBE",
        "params":
        [
            "btcusdt@bookTicker",
            "bnbusdt@bookTicker"
        ],
        "id": 1
    }"#.into())).expect("Uh oh!");

    

    thread::spawn(move || {
        loop {
            // println!("Other thread looping");
            for(key, value) in second_thread.iter() {
               println!("[OTHER THREAD] {} is trading at: {}", key, value.asking_price);
            }
            thread::sleep(time::Duration::from_millis(100));
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
        
        let tick = Ticker::new(msg_as_string).unwrap_or_else(|_err| {
            process::exit(1)
        });

        println!("[MAIN THREAD] {} is at an asking price of {}", tick.symbol, tick.asking_price);
        unsafe {
            let t = Arc::get_mut_unchecked(&mut main_thread);
            t.insert(tick.symbol.clone(), tick);
        }

    }

}
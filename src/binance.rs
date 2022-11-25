use crate::path::Path;
use tungstenite::{WebSocket, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use url::Url;
use tungstenite::{connect};

pub struct Binance {
    pub paths: Vec<Path>,
    pub connection: WebSocket<MaybeTlsStream<TcpStream>>,
    pub api_key: String,
}

impl Binance {

    pub fn new(url: Url, api_key: String, paths: Vec<Path>) -> Binance {
        Binance {
            paths,
            connection: Binance::connect(url),
            api_key
        }
    }

    pub fn connect(url: Url) -> WebSocket<MaybeTlsStream<TcpStream>> {
        let (mut socket, _response) = connect(
            url
        ).expect("Couldn't connect to socket");
        socket
    }

    // pub fn send_message(msg: Message) -> {

    // }
}
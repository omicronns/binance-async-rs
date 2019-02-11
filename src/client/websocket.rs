use std::collections::HashMap;

use failure::Error;
use futures::stream::{SplitStream, Stream};
use futures::{Future, Poll};
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::client::Binance;
use crate::error::{BinanceError, Result};
use crate::model::websocket::{
    AccountUpdate, BinanceSubscription, BinanceWebsocketMessage, OrderUpdate,
};

const WS_URL: &'static str = "wss://stream.binance.com:9443/ws";

impl Binance {
    pub fn websocket(&self) -> BinanceWebsocket {
        BinanceWebsocket {
            subscriptions: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket {
    subscriptions: HashMap<BinanceSubscription, SplitStream<WSStream>>,
}

impl BinanceWebsocket {
    pub fn subscribe(
        mut self,
        subscription: BinanceSubscription,
    ) -> impl Future<Item = Self, Error = Error> {
        let sub = match subscription {
            BinanceSubscription::AggregateTrade(ref symbol) => format!("{}@aggTrade", symbol),
            BinanceSubscription::Trade(ref symbol) => format!("{}@trade", symbol),
            BinanceSubscription::Candlestick(ref symbol, ref interval) => {
                format!("{}@kline_{}", symbol, interval)
            }
            BinanceSubscription::MiniTicker(ref symbol) => format!("{}@miniTicker", symbol),
            BinanceSubscription::MiniTickerAll => "!miniTicker@arr".to_string(),
            BinanceSubscription::Ticker(ref symbol) => format!("{}@ticker", symbol),
            BinanceSubscription::TickerAll => "!ticker@arr".to_string(),
            BinanceSubscription::OrderBook(ref symbol, depth) => {
                format!("{}@depth{}", symbol, depth)
            }
            BinanceSubscription::DiffDepth(ref symbol) => format!("{}@depth", symbol),
            BinanceSubscription::UserData(ref listen_key) => listen_key.clone(),
        };

        trace!("[Websocket] Subscribing to '{:?}'", subscription);

        let endpoint = Url::parse(&format!("{}/{}", WS_URL, sub)).unwrap();
        connect_async(endpoint)
            .map(|(stream, _)| stream)
            .map(|s| s.split().1)
            .map(|stream| {
                self.subscriptions.insert(subscription, stream);
                self
            })
            .from_err()
    }

    pub fn unsubscribe(
        &mut self,
        subscription: &BinanceSubscription,
    ) -> Option<SplitStream<WSStream>> {
        self.subscriptions.remove(subscription)
    }
}

impl Stream for BinanceWebsocket {
    type Item = BinanceWebsocketMessage;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let streams: Vec<_> = self
            .subscriptions
            .iter_mut()
            .map(|(sub, stream)| {
                stream
                    .from_err()
                    .and_then(move |msg| parse_message(sub.clone(), msg))
            })
            .collect();

        let streams = streams.into_iter().fold(
            None,
            |acc: Option<Box<Stream<Item = BinanceWebsocketMessage, Error = Error>>>, elem| {
                match acc {
                    Some(stream) => Some(Box::new(stream.select(elem.from_err()))),
                    None => Some(Box::new(elem.from_err())),
                }
            },
        );
        match streams {
            Some(mut streams) => streams.poll(),
            None => Err(BinanceError::NoStreamSubscribed)?,
        }
    }
}

fn parse_message(sub: BinanceSubscription, msg: Message) -> Result<BinanceWebsocketMessage> {
    let msg = match msg {
        Message::Text(msg) => msg,
        Message::Binary(b) => return Ok(BinanceWebsocketMessage::Binary(b)),
        Message::Pong(..) => return Ok(BinanceWebsocketMessage::Pong),
        Message::Ping(..) => return Ok(BinanceWebsocketMessage::Ping),
    };

    trace!("Incoming websocket message {}", msg);
    let message = match sub {
        BinanceSubscription::AggregateTrade(..) => {
            BinanceWebsocketMessage::AggregateTrade(from_str(&msg)?)
        }
        BinanceSubscription::Trade(..) => BinanceWebsocketMessage::Trade(from_str(&msg)?),
        BinanceSubscription::Candlestick(..) => {
            BinanceWebsocketMessage::Candlestick(from_str(&msg)?)
        }
        BinanceSubscription::MiniTicker(..) => BinanceWebsocketMessage::MiniTicker(from_str(&msg)?),
        BinanceSubscription::MiniTickerAll => {
            BinanceWebsocketMessage::MiniTickerAll(from_str(&msg)?)
        }
        BinanceSubscription::Ticker(..) => BinanceWebsocketMessage::Ticker(from_str(&msg)?),
        BinanceSubscription::TickerAll => BinanceWebsocketMessage::TickerAll(from_str(&msg)?),
        BinanceSubscription::OrderBook(..) => BinanceWebsocketMessage::OrderBook(from_str(&msg)?),
        BinanceSubscription::DiffDepth(..) => BinanceWebsocketMessage::DiffDepth(from_str(&msg)?),
        BinanceSubscription::UserData(..) => {
            let msg: Either<AccountUpdate, OrderUpdate> = from_str(&msg)?;
            match msg {
                Either::Left(m) => BinanceWebsocketMessage::AccountUpdate(m),
                Either::Right(m) => BinanceWebsocketMessage::OrderUpdate(m),
            }
        }
    };
    Ok(message)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

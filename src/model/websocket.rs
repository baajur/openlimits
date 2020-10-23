use crate::exchange::ExchangeSpec;

use super::{OrderBookResponse, Trade};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    UserData(String),            // listen key
    AggregateTrade(String),      // symbol
    Trade(String),               // symbol
    Candlestick(String, String), // (symbol, interval)
    MiniTicker(String),          // symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    OrderBook(String, i64),     // (symbol, depth)
    Depth(String, Option<u16>), // (symbol, interval)
}

#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebsocketMessage<E: ExchangeSpec> {
    Ping,
    OrderBook(OrderBookResponse),
    Trades(Vec<Trade<E>>),
}

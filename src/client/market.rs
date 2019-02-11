use std::collections::HashMap;
use std::iter::FromIterator;

use failure::Error;
use futures::Future;
use serde_json::json;
use serde_json::Value;

use super::Binance;
use decimal::Decimal;
use error::Result;
use model::{KlineMsg, OrderBookMsg, PriceMsg, PriceStatsMsg, TickerMsg};

// Market Data endpoints
impl Binance {
    // Order book (Default 100; max 1000)
    pub fn get_depth<I>(
        &self,
        symbol: &str,
        limit: I,
    ) -> Result<impl Future<Item = OrderBookMsg, Error = Error>>
    where
        I: Into<Option<u64>>,
    {
        let limit = limit.into().unwrap_or(100);
        let params = json! {{"symbol": symbol, "limit": limit}};

        Ok(self.transport.get("/api/v1/depth", Some(params))?)
    }

    // Latest price for ONE symbol.
    pub fn get_price(&self, symbol: &str) -> Result<impl Future<Item = PriceMsg, Error = Error>> {
        let params = json! {{"symbol": symbol}};
        Ok(self.transport.get("/api/v3/ticker/price", Some(params))?)
    }

    // Latest price for ALL symbols.
    pub fn get_price_all(&self) -> Result<impl Future<Item = Vec<PriceMsg>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/api/v3/ticker/price", None)?)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker(
        &self,
        symbol: &str,
    ) -> Result<impl Future<Item = TickerMsg, Error = Error>> {
        let params = json! {{"symbol": symbol}};
        Ok(self
            .transport
            .get("/api/v3/ticker/bookTicker", Some(params))?)
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_book_ticker_all(&self) -> Result<impl Future<Item = Vec<TickerMsg>, Error = Error>> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v3/ticker/bookTicker", None)?)
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats(
        &self,
        symbol: &str,
    ) -> Result<impl Future<Item = PriceStatsMsg, Error = Error>> {
        let params = json! {{"symbol": symbol}};
        Ok(self.transport.get("/api/v1/ticker/24hr", Some(params))?)
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats_all(
        &self,
    ) -> Result<impl Future<Item = Vec<PriceStatsMsg>, Error = Error>> {
        Ok(self.transport.get::<_, ()>("/api/v1/ticker/24hr", None)?)
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub fn get_klines<S3, S4, S5>(
        &self,
        symbol: &str,
        interval: &str,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<impl Future<Item = Vec<KlineMsg>, Error = Error>>
    where
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut params = vec![
            ("symbol", symbol.to_string()),
            ("interval", interval.to_string()),
        ];

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            params.push(("limit", lt.to_string()));
        }
        if let Some(st) = start_time.into() {
            params.push(("startTime", st.to_string()));
        }
        if let Some(et) = end_time.into() {
            params.push(("endTime", et.to_string()));
        }
        let params: HashMap<&str, String> = HashMap::from_iter(params);

        // todo: check if kline parsing could be done with serde
        // Ok(self.transport.get("/api/v1/klines", Some(params))?)

        let klines =
            self.transport
                .get("/api/v1/klines", Some(params))?
                .map(|data: Vec<Vec<Value>>| {
                    data.iter()
                        .map(|row| KlineMsg {
                            open_time: to_u64(&row[0]),
                            open: to_decimal(&row[1]),
                            high: to_decimal(&row[2]),
                            low: to_decimal(&row[3]),
                            close: to_decimal(&row[4]),
                            volume: to_decimal(&row[5]),
                            close_time: to_u64(&row[6]),
                            quote_asset_volume: to_decimal(&row[7]),
                            number_of_trades: to_u64(&row[8]),
                            taker_buy_base_asset_volume: to_decimal(&row[9]),
                            taker_buy_quote_asset_volume: to_decimal(&row[10]),
                        })
                        .collect()
                });
        Ok(klines)
    }
}

fn to_u64(v: &Value) -> u64 {
    v.as_u64().unwrap()
}

fn to_decimal(v: &Value) -> Decimal {
    v.as_str().unwrap().parse().unwrap()
}

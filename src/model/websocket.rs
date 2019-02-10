use super::{Asks, Bids, OrderBook};
use decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinanceSubscription {
    // Websocket streams
    AggregateTrade(String),      //symbol
    Trade(String),               //symbol
    Candlestick(String, String), //symbol, interval
    MiniTicker(String),          //symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    PartialDepth(String, u64), //symbol, level
    DiffDepth(String),         //symbol
    OrderBook(String, u64),    //symbol, depth

    // User data streams
    UserData(String), // listen key
}

#[derive(Debug, Clone, Serialize)]
pub enum BinanceWebsocketMessage {
    // Websocket streams
    AggregateTrade(AggregateTrade),
    Trade(Trade),
    Candlestick(CandlestickMessage),
    MiniTicker(MiniTicker),
    MiniTickerAll(Vec<MiniTicker>),
    Ticker(Ticker),
    TickerAll(Vec<Ticker>),
    OrderBook(OrderBook),
    PartialDepth(PartialDepth),
    DiffDepth(DiffDepth),
    Ping,
    Pong,

    // User data streams
    AccountUpdate(AccountUpdate),
    OrderUpdate(OrderUpdate),

    Binary(Vec<u8>), // Unexpected, unparsed
}

// Websocket streams
// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTrade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M", skip_serializing)]
    pub m_ignore: bool,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#trade-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub qty: Decimal,
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M", skip_serializing)]
    pub m_ignore: bool,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#klinecandlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: CandlestickMessageKline,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#klinecandlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickMessageKline {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "o")]
    pub open_price: Decimal,
    #[serde(rename = "c")]
    pub close_price: Decimal,
    #[serde(rename = "h")]
    pub high_price: Decimal,
    #[serde(rename = "l")]
    pub low_price: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    #[serde(rename = "x")]
    pub is_kline_closed: bool,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "V")]
    pub taker_buy_base_volume: Decimal,
    #[serde(rename = "Q")]
    pub taker_buy_quote_volume: Decimal,
    #[serde(rename = "B", skip_serializing)]
    pub b_ignore: String,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#individual-symbol-mini-ticker-stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub close_price: Decimal,
    #[serde(rename = "o")]
    pub open_price: Decimal,
    #[serde(rename = "l")]
    pub low_price: Decimal,
    #[serde(rename = "h")]
    pub high_price: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#individual-symbol-ticker-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: Decimal,
    #[serde(rename = "P")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w")]
    pub average_price: Decimal,
    #[serde(rename = "x")]
    pub first_price: Decimal,
    #[serde(rename = "c")]
    pub last_price: Decimal,
    #[serde(rename = "Q")]
    pub last_qty: Decimal,
    #[serde(rename = "b")]
    pub best_bid_price: Decimal,
    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,
    #[serde(rename = "a")]
    pub best_ask_price: Decimal,
    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,
    #[serde(rename = "o")]
    pub open_price: Decimal,
    #[serde(rename = "h")]
    pub high_price: Decimal,
    #[serde(rename = "l")]
    pub low_price: Decimal,
    #[serde(rename = "v")]
    pub base_volume: Decimal,
    #[serde(rename = "q")]
    pub quote_volume: Decimal,
    #[serde(rename = "O")]
    pub stat_open_time: u64,
    #[serde(rename = "C")]
    pub stat_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#partial-book-depth-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialDepth {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/web-socket-streams.md#diff-depth-stream
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiffDepth {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bids>,
    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

// User data streams
// https://github.com/binance-exchange/binance-official-api-docs/blob/master/user-data-stream.md

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/user-data-stream.md#account-update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "m")]
    pub maker_commision_rate: u64,
    #[serde(rename = "t")]
    pub taker_commision_rate: u64,
    #[serde(rename = "b")]
    pub buyer_commision_rate: u64,
    #[serde(rename = "s")]
    pub seller_commision_rate: u64,
    #[serde(rename = "T")]
    pub can_trade: bool,
    #[serde(rename = "W")]
    pub can_withdraw: bool,
    #[serde(rename = "D")]
    pub can_deposit: bool,
    #[serde(rename = "u")]
    pub last_update_time: u64,
    #[serde(rename = "B")]
    pub balances: Vec<AccountUpdateBalance>,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/user-data-stream.md#account-update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: Decimal,
    #[serde(rename = "l")]
    pub locked: Decimal,
}

// https://github.com/binance-exchange/binance-official-api-docs/blob/master/user-data-stream.md#order-update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: OrderTimeInForce,
    #[serde(rename = "q")]
    pub order_qty: Decimal,
    #[serde(rename = "p")]
    pub order_price: Decimal,
    #[serde(rename = "P")]
    pub stop_price: Decimal,
    #[serde(rename = "F")]
    pub iceberg_qty: Decimal,
    #[serde(skip_serializing)]
    pub g: i64,
    #[serde(rename = "C")]
    pub orig_client_order_id: Option<String>,
    #[serde(rename = "x")]
    pub execution_type: OrderExecType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "r")]
    pub order_reject_reason: OrderRejectReason,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l")]
    pub last_exec_qty: Decimal,
    #[serde(rename = "z")]
    pub cumulative_filled_qty: Decimal,
    #[serde(rename = "L")]
    pub last_exec_price: Decimal,
    #[serde(rename = "n")]
    pub commission_qty: Decimal,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "I", skip_serializing)]
    pub i_ignore: i64,
    #[serde(rename = "w")]
    pub is_working: bool,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "M", skip_serializing)]
    pub m_ignore: bool,
    #[serde(rename = "O")]
    pub order_creation_time: u64,
    #[serde(rename = "Z")]
    pub cumulative_quote_transacted_qty: Decimal,
    #[serde(rename = "Y")]
    pub last_quote_transacted_qty: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecType {
    New,
    Canceled,
    Replaced,
    Rejected,
    Trade,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRejectReason {
    None,
}

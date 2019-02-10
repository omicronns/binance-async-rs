extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate tokio;

use std::env::var;

use tokio::runtime::current_thread::Runtime;

use binance::error::Result;
use binance::Binance;

fn main() -> Result<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();
    let api_key = var("BINANCE_KEY")?;
    let secret_key = var("BINANCE_SECRET")?;

    let mut rt = Runtime::new()?;
    let bn = Binance::with_credential(&api_key, &secret_key);

    // General
    match rt.block_on(bn.ping()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.get_server_time()?) {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.get_exchange_info()?) {
        Ok(answer) => println!("Exchange info: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Account
    match rt.block_on(bn.get_account()?) {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.get_all_open_orders()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.limit_buy("ETHBTC", 1.into(), "0.1".parse().unwrap())?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.market_buy("WTCETH", 5.into())?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.limit_sell("WTCETH", 10.into(), "0.035".parse().unwrap())?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.market_sell("WTCETH", 5.into())?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.order_status("WTCETH", 1_957_528)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.cancel_order("WTCETH", 1_957_528)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.get_balance("KNC")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match rt.block_on(bn.trade_history("WTCETH")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Market

    // Order book
    match rt.block_on(bn.get_depth("BNBETH", None)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match rt.block_on(bn.get_all_prices()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match rt.block_on(bn.get_price("KNCETH")?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match rt.block_on(bn.get_all_book_tickers()?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match rt.block_on(bn.get_book_ticker("BNBETH")?) {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match rt.block_on(bn.get_24h_price_stats("BNBETH")?) {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match rt.block_on(bn.get_klines("BNBETH", "5m", 10, None, None)?) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

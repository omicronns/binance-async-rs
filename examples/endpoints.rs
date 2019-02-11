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

    println!("Ping:");
    println!("{:?}", rt.block_on(bn.ping()?)?);
    println!("Server Time:");
    println!("{:?}", rt.block_on(bn.get_server_time()?)?);
    println!("Exchange info:");
    println!("{:?}", rt.block_on(bn.get_exchange_info()?)?);
    println!("Account:");
    println!("{:?}", rt.block_on(bn.get_account()?)?);
    println!("Get all open orders:");
    println!("{:?}", rt.block_on(bn.get_open_orders_all()?)?);
    println!("Get open orders:");
    println!("{:?}", rt.block_on(bn.get_open_orders("TRXBTC")?)?);

    // println!("Limit buy:");
    // println!(
    //     "{:?}",
    //     rt.block_on(bn.limit_buy("ETHBTC", 1.into(), "0.1".parse().unwrap())?)?
    // );
    // println!("Market buy:");
    // println!("{:?}", rt.block_on(bn.market_buy("WTCETH", 5.into())?)?);
    // println!("Limit sell:");
    // println!(
    //     "{:?}",
    //     rt.block_on(bn.limit_sell("WTCETH", 10.into(), "0.035".parse().unwrap())?)?
    // );
    // println!("Limit buy:");
    // println!("{:?}", rt.block_on(bn.market_sell("WTCETH", 5.into())?)?);
    // println!("Order status:");
    // println!("{:?}", rt.block_on(bn.order_status("WTCETH", 1_957_528)?)?);
    // println!("Cancel order:");
    // println!("{:?}", rt.block_on(bn.cancel_order("WTCETH", 1_957_528)?)?);

    println!("Get balance:");
    println!("{:?}", rt.block_on(bn.get_balance("ETH")?)?);
    println!("Trade history:");
    println!("{:?}", rt.block_on(bn.trade_history("WTCETH")?)?);
    println!("Get depth:");
    println!("{:?}", rt.block_on(bn.get_depth("BNBETH", None)?)?);
    println!("Get all prices:");
    println!("{:?}", rt.block_on(bn.get_price_all()?)?);
    println!("Get price:");
    println!("{:?}", rt.block_on(bn.get_price("KNCETH")?)?);
    println!("Get all tickers:");
    println!("{:?}", rt.block_on(bn.get_book_ticker_all()?)?);
    println!("Get ticker:");
    println!("{:?}", rt.block_on(bn.get_book_ticker("BNBETH")?)?);
    println!("Get 24h price stats:");
    println!("{:?}", rt.block_on(bn.get_24h_price_stats("BNBETH")?)?);
    println!("Get klines:");
    println!(
        "{:?}",
        rt.block_on(bn.get_klines("BNBETH", "5m", 10, None, None)?)?
    );

    Ok(())
}

use markets::*;

// FIXME: API looks strange? think/read about it!

fn main() {
    let mut market = Market::new();
    market.get_klines();
    // TODO: maybe incapsulate into market.print_candles()?
    market
        .candles
        .read()
        .unwrap()
        .clone()
        .iter()
        .for_each(|Candle { high, low, .. }| println!("high: {high:<20.2} low: {low:<.2}"))
}

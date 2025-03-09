#[allow(unused, dead_code)]
// binance:
use binance::api::*;
use binance::market::*;
use binance::model::*;

pub struct Store {
    pub prices: Vec<SymbolPrice>,
    pub market: Market,
    pub news: Vec<(String, String)>,
}

impl Default for Store {
    fn default() -> Self {
        let market: Market = Binance::new(None, None);
        let all_prices = match market.get_all_prices().unwrap() {
            //TODO: Handle error cases
            Prices::AllPrices(a) => a,
        };

        let prices = all_prices.into_iter().filter(|p| p.price > 1.0).collect();

        let news = vec![
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
            ("Heading".to_string(), "information".to_string()),
        ];

        Self {
            prices,
            market,
            news,
        }
    }
}

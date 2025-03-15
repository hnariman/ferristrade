use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[allow(unused, dead_code)]
// binance:
use binance::api::*;
use binance::market::*;
use binance::model::*;
use news::{self, Article};

pub struct Store {
    pub prices: Vec<SymbolPrice>,
    pub market: Market,
    // so that we can share data between threads (egui/update)
    pub news: Arc<Mutex<HashSet<Article>>>,
    pub chart_data: Vec<KlineSummary>,
}

impl Default for Store {
    fn default() -> Self {
        let market: Market = Binance::new(None, None);
        let all_prices = match market.get_all_prices().expect("market get_all_prices") {
            //TODO: Handle error cases
            Prices::AllPrices(a) => a,
        };

        let prices = all_prices.into_iter().filter(|p| p.price > 1.0).collect();

        let feeds = news::Feeds::default();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(feeds.update());
        dbg!(feeds.news);
        let news = Arc::new(Mutex::new(HashSet::new()));

        let chart_data = match market.get_klines("BTCUSDT", "1d", 100, None, None).unwrap() {
            KlineSummaries::AllKlineSummaries(v) => v,
        };

        Self {
            prices,
            market,
            news,
            chart_data,
        }
    }
}

impl Store {
    pub fn get_news(&self) -> Result<(), Box<dyn std::error::Error>> {
        let feeds = news::Feeds::default();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(feeds.update());
        *self.news.lock().unwrap() = feeds.news.read().unwrap().clone();

        Ok(())
    }

    pub fn update_tickers(&mut self) {
        let market: Market = Binance::new(None, None);
        let all_prices = match market.get_all_prices().expect("market get_all_prices") {
            Prices::AllPrices(a) => a,
        };

        self.prices = all_prices.into_iter().filter(|p| p.price > 1.0).collect();
    }

    // pub fn update_news(&self, store: Arc<Mutex<Store>>) {
    // thread::spawn(move || loop {
    //     let state_clone = state.clone();
    //     let state_guard = state_clone.lock().unwrap();
    //     if let Err(e) = state_guard.get_news() {
    //         eprint!("Houston, we have a problem updating news: {e}");
    //     }
    //     drop(state_guard);
    //     thread::sleep(Duration::from_secs(60));
    // });

    //     self.rt.spawn(async move {
    //         let mut interval = interval(Duration::from_secs(60));
    //         loop {
    //             interval.tick().await;
    //             let store_clone = store.clone();
    //             let guard = store_clone.lock().unwrap();
    //             if let Err(e) = guard.get_news() {
    //                 eprintln!("Error getting news: {}", e);
    //             }
    //         }
    //     });
    //     ()
    // }
}

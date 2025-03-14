use std::sync::{Arc, Mutex};
use std::time::Duration;

#[allow(unused, dead_code)]
// binance:
use binance::api::*;
use binance::market::*;
use binance::model::*;
use tokio::runtime;
use tokio::time::interval;

pub struct Store {
    pub rt: tokio::runtime::Runtime,
    pub prices: Vec<SymbolPrice>,
    pub market: Market,
    // so that we can share data between threads (egui/update)
    pub news: Arc<Mutex<Vec<rss::Item>>>,
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

        let news = Arc::new(Mutex::new(Vec::new()));
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let chart_data = match market.get_klines("BTCUSDT", "1d", 100, None, None).unwrap() {
            KlineSummaries::AllKlineSummaries(v) => v,
        };

        Self {
            rt,
            prices,
            market,
            news,
            chart_data,
        }
    }
}

impl Store {
    fn get_news(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines";
        let res = reqwest::blocking::get(url)?.bytes()?;
        let channel = rss::Channel::read_from(&res[..])?;
        let mut items = self.news.lock().unwrap();
        dbg!(&channel);
        *items = channel.items().to_vec();
        Ok(())
    }

    pub fn update_news(&self, store: Arc<Mutex<Store>>) {
        // thread::spawn(move || loop {
        //     let state_clone = state.clone();
        //     let state_guard = state_clone.lock().unwrap();
        //     if let Err(e) = state_guard.get_news() {
        //         eprint!("Houston, we have a problem updating news: {e}");
        //     }
        //     drop(state_guard);
        //     thread::sleep(Duration::from_secs(60));
        // });

        self.rt.spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let store_clone = store.clone();
                let guard = store_clone.lock().unwrap();
                if let Err(e) = guard.get_news() {
                    eprintln!("Error getting news: {}", e);
                }
            }
        });
        ()
    }
}

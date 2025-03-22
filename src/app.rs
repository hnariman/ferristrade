use std::collections::HashSet;

use crate::config::Config;
use crate::store::Store;
use crate::ui::{
    central_panel::central_panel, left_panel::left_panel, menu_panel::app_menu,
    right_panel::right_panel,
};
use binance::api::Binance;
use binance::model::{KlineSummaries, KlineSummary};
use eframe::egui;
use news::Article;

pub struct Terminal {
    store: Store,
    config: Config,
}

impl eframe::App for Terminal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);
        app_menu(ctx);
        left_panel(ctx, &mut self.config);
        right_panel(ctx);
        central_panel(ctx);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        let (market_tx, market_rx) = tokio::sync::mpsc::channel::<Vec<KlineSummary>>(100);
        let (news_tx, news_rx) = tokio::sync::mpsc::channel::<HashSet<Article>>(100);

        let config = Config::default();
        let store = Store::new(news_rx, market_rx);

        // TODO: shall this go to update method?
        tokio::task::spawn(async move {
            market_tx.send(get_tickers().await).await.unwrap();
        });

        tokio::task::spawn(async move {
            news_tx.send(get_news().await).await.unwrap();
        });

        Self { store, config }
    }
}

pub async fn get_tickers() -> Vec<KlineSummary> {
    //TODO: Market::get_tickers()
    let market: binance::market::Market = Binance::new(None, None);
    let result = market
        .get_klines("BTCUSDT", "15m", 10, None, None)
        .expect("klines request failed");

    match result {
        KlineSummaries::AllKlineSummaries(data) => {
            return data;
        }
    };
}

pub async fn get_news() -> HashSet<Article> {
    //TODO: Feeds::get_news()
    let url = "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines";

    let response = reqwest::get(url).await.unwrap();
    let content = response.bytes().await.unwrap();
    let channel = rss::Channel::read_from(&content[..]).unwrap();

    let articles = channel
        .items
        .iter()
        .map(|a| Article::new(&a.title.as_ref().unwrap()))
        .collect::<HashSet<Article>>();
    articles
}

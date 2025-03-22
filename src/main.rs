mod app;
mod ui;
use app::Terminal;
use binance::model::KlineSummary;
use news::Article;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    // async fn main() -> eframe::Result {
    // let _feeds = news::Feeds::default();
    // let _market = markets::Market::new();

    let (market_tx, mut market_rx) = tokio::sync::mpsc::channel::<Vec<KlineSummary>>(100);
    let (news_tx, _news_rx) = tokio::sync::mpsc::channel::<Vec<Article>>(100);

    // tokio::task::spawn(news());
    // tokio::task::spawn(market());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(), //.with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferristrade",
        options,
        Box::new(|_cc| Ok(Box::new(Terminal::default()))),
    )
}

// #[derive(Debug, thiserror::Error)]
// enum ApplicationError {
//     #[error("problem in eframe UI")]
//     UIError,
// }

// async fn news() {
//     async move {
//         let url = "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines";
//         let response = reqwest::get(url).await.expect("no request response");
//         let content = response
//             .bytes()
//             .await
//             .expect("unable to convert request to bytes");
//         let channel =
//             rss::Channel::read_from(&content[..]).expect("unable to read request channel");

//         let result = channel
//             .items
//             .iter()
//             .map(|a| {
//                 news::Article::new(
//                     &a.title
//                         .as_ref()
//                         .expect("unable to get results from channel"),
//                 )
//             })
//             .collect::<Vec<Article>>();

//         dbg!(&result);
//         news_tx.send(result).await.expect("not able to send news");
//     }
// }

// async fn market() {
//     async move {
//         let req = tokio::task::spawn_blocking(move || {
//             let mut klines: Vec<KlineSummary> = vec![];
//             let market: binance::market::Market = Binance::new(None, None);
//             dbg!("getting market");

//             let response = market.get_klines("BTCUSDT", "15m", 10, None, None);
//             dbg!(&response);

//             if let Ok(data) = response {
//                 match data {
//                     KlineSummaries::AllKlineSummaries(v) => {
//                         dbg!(&v);
//                         klines.extend_from_slice(&v);
//                     }
//                 }
//             }
//         });

//         let klines = req.into_future().await.unwrap();
//         dbg!(klines);

//         // dbg!(&klines);
//         // market_tx
//         //     .send(klines)
//         //     .await
//         //     .expect("not able to send markets data tx");
//     }
// }

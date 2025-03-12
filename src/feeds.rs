use reqwest;
use rss;
#[derive(Debug, Default)]
struct Article {
    title: String,
    text: String,
}

#[derive(Debug)]
pub struct Feeds {
    // dow: Vec<Article>,
}

impl Default for Feeds {
    fn default() -> Self {
        let _url = "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines";
        // let dow = Feeds::get_news(url.to_string());

        Self {
            // dow: vec![Article::default()],
        }
    }
}

impl Feeds {
    pub async fn get_news() {
        let url = "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines";
        let response = reqwest::get(url).await.unwrap();
        let content = response.bytes().await.unwrap();
        let channel = rss::Channel::read_from(&content[..]).unwrap();
        channel.items();
    }
}

#![allow(unused, dead_code)] //FIXME: remove when done!
use std::{
    collections::HashSet,
    result::Result,
    sync::{Arc, RwLock},
};

use reqwest::{self};
use rss;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Article {
    title: String,
}

#[derive(Debug, Default, Clone)]
pub struct Resource {
    url: String,
    title: String,
    website: String,
}

impl Resource {
    fn new(url: &str, title: &str, website: Option<&str>) -> Self {
        Self {
            url: String::from(url),
            title: String::from(title),
            website: website.map_or(String::from(""), String::from),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Feeds {
    pub news: Arc<RwLock<HashSet<Article>>>,
    pub urls: Arc<RwLock<Vec<Resource>>>,
}

impl Default for Feeds {
    fn default() -> Self {
        Self {
            news: Arc::new(RwLock::new(HashSet::default())),
            urls: Arc::new(RwLock::new(vec![
                // TODO: move to yml/toml/json config maybe even env
                Resource::new(
                    "https://feeds.content.dowjones.io/public/rss/mw_realtimeheadlines",
                    "DowJones",
                    None,
                ),
                Resource::new("https://www.ft.com/rss/home/uk", "Financial Times", None),
                // Resource::new(
                //     "https://www.wsj.com/news/rss-news-and-feeds",
                //     "The Wall Street Journal",
                //     None,
                // ),
                Resource::new(
                    "http://rss.cnn.com/rss/money_latest.rss",
                    "CNN Business",
                    None,
                ),
                // Resource::new(
                //     "http://feeds.reuters.com/reuters/businessNews",
                //     "Reuters",
                //     None,
                // ),
                // Resource::new(
                //     "https://www.cnbc.com/id/10000311/device/rss/rss.html",
                //     "CNBC",
                //     None,
                // ),
                // Resource::new(
                //     "https://www.bloomberg.com/feeds/markets/rss",
                //     "Bloomberg",
                //     None,
                // ),
                Resource::new(
                    "https://finance.yahoo.com/rss/topstories",
                    "Yahoo Finance",
                    None,
                ),
            ])),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FeedsError {
    #[error("some network error")]
    Request(#[from] reqwest::Error),
    #[error("formatting error")]
    Formatting(#[from] std::fmt::Error),
    #[error("RSS Error")]
    RSS(#[from] rss::Error),
    #[error("mutex lock poisoned")]
    MutexPoisoned,
}

impl Feeds {
    pub async fn fetch(resource: &Resource) -> Result<HashSet<Article>, FeedsError> {
        let response = reqwest::get(&resource.url).await?;
        let content = response.bytes().await?;
        let channel = rss::Channel::read_from(&content[..])?;

        let articles = channel
            .items
            .iter()
            .map(|a| {
                dbg!(a);
                return Article {
                    title: a.title.clone().unwrap_or_default(),
                };
            })
            .collect::<HashSet<Article>>();

        Ok(articles)
    }

    //TODO: and now implement method which doesn't use tokio sleep/timer,
    // egui has own timer :palmface:
    // next time RTFM and Docs :)

    pub async fn update_and_show(&self) {
        let feed = self.clone();

        tokio::spawn(async move {
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(10));

            loop {
                let mut accumulated: HashSet<Article> = HashSet::new();

                timer.tick().await;

                let news_guard = feed.urls.read().unwrap().clone();
                for resource in news_guard.iter() {
                    if let Ok(v) = Self::fetch(resource).await {
                        accumulated = accumulated.union(&v).cloned().collect();
                    }
                }
                // let data = news_guard
                //     .iter()
                //     .map(async |r| Self::fetch(r).await)
                //     .collect();
                // accumulated = accumulated.union(data).cloned().collect();

                let mut news = feed.news.write().unwrap().clone();
                news = news.union(&accumulated).cloned().collect();

                // news.iter().for_each(|n| {
                //     println!("{}", n.title);
                // });
                println!("{}", "-".repeat(80));
            }
        });
    }
}

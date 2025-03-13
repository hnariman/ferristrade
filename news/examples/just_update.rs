use news::Feeds;

fn main() {
    let feeds = Feeds::default();
    let news = &feeds.news.read().unwrap().clone();
    dbg!(news);

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(feeds.update());

    let news = &feeds.news.read().unwrap().clone();
    dbg!(news);
}

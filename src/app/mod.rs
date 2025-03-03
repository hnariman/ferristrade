pub mod app {
    pub struct MyApp {
        zoom: f32,
        prices: binance::model::Prices,
    }

    impl Default for MyApp {
        fn default() -> Self {
            let market: Market = Binance::new(None, None);
            let prices = market.get_all_prices().unwrap();
            Self { prices, zoom: 1.0 }
        }
    }

    impl MyApp {
        fn zoomin(&mut self) {
            if self.zoom > 4.0 {
                return;
            }
            self.zoom += 0.2;
        }

        fn zoomout(&mut self) {
            if self.zoom < 1.0 {
                return;
            }
            self.zoom -= 0.2;
        }
    }
}

#![allow(dead_code)]

use core::f64;
use std::sync::{Arc, RwLock};

use binance::{
    self,
    api::Binance,
    market::Market as BinanceMarket,
    model::{KlineSummaries, KlineSummary},
};

#[derive(thiserror::Error, Debug)]
pub enum MarketError {
    #[error("unable to parse")]
    Parsing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

//FIXME: Binance doesn't have Default & Debug
// I need to find way to extend it
// or I need to replace binance with reqwest sooner
#[derive(Clone)]
pub struct Market {
    pub candles: Arc<RwLock<Vec<Candle>>>,
    market: BinanceMarket,
}

impl Market {
    pub fn new() -> Self {
        Market {
            candles: Arc::new(RwLock::new(vec![])),
            market: Binance::new(None, None),
        }
    }

    pub fn get_klines(&mut self) {
        let mut klines: Vec<KlineSummary> = vec![];

        let response = self.market.get_klines("BTCUSDT", "15m", 10, None, None);

        if let Ok(v) = response {
            match v {
                KlineSummaries::AllKlineSummaries(res) => klines.extend(res),
            }
        }

        if let Ok(data) = klines.into_iter().map(Candle::try_from).collect() {
            self.candles.write().unwrap().extend::<Vec<Candle>>(data)
        }
    }
    fn parse(s: &str) -> Result<f64, MarketError> {
        s.parse::<f64>().map_err(|_| MarketError::Parsing)
    }
}

impl TryFrom<KlineSummary> for Candle {
    type Error = MarketError;
    fn try_from(k: KlineSummary) -> Result<Self, Self::Error> {
        let open = Market::parse(&k.open)?;
        let close = Market::parse(&k.close)?;
        let high = Market::parse(&k.high)?;
        let low = Market::parse(&k.low)?;
        let volume = Market::parse(&k.volume)?;

        Ok(Candle {
            open,
            high,
            close,
            low,
            volume,
        })
    }
}

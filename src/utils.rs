// pub type Result = eframe::Result;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]

pub enum MarketError {
    #[error("Failed to get price")]
    FailedToGetPrice,
}

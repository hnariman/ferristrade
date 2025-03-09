# About

A basic usecase for egui + binance-rs

An attempt to build realtime Stock & Derrivatives Terminal

(after thinking of AI I'm probably closing the source :)

## In Future:

- Add Axum or AWS Lambda for auth (preferrably use some federated auth through AWS Cognito)
- Alternate binance-rs with reqwest & tokio_stram for realtime data (to prevent vendor lock to binance)
- Embed egui as wasm - so no installation is requried
- Chart tools & indicators (MA,RSI, Bollinger Bands, Trends)
- Candlestick formation indicators
- Add tables for options trading and options strategy analysis (such as Iron Condors, Butterflies, Collars etc)
- implement polynomial, linear regression analysis (more onto quantitative trading statistical analysis)
- Backend (aws lambda + DynamoDB) - to store user settings/trading performance, maybe strategies and summary


### Backtesting

Ability to backtest strategies using historical data and simulate trading performance.
use AI for summary generation and trading strategy optimization.

### AI

- use AI for summary generation and trading strategy optimization.(distil-Berta for sentiment)
- ability to summarize RSS (only users selection/subscriptions) news and show summary card on UI for the current TICKER sentiment
- ability to use some HuggingFace models for market data analysis and sentiment analysis
- stocks can be added in the watchlist and AI provides sentiment analysis for each stock and "portfolio"
- rss/twitter sentiment analysis for tweets related to stocks and portfolios

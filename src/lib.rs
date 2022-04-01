pub mod api;
pub mod client;
pub mod error;
pub mod modules;

use api::Api;
pub use client::Client;
pub use client::Response;
pub use error::Error;
pub mod entities{pub mod market;pub mod orderbook;pub mod trade;pub mod liquidity_providers;pub mod stats;}

pub type Result<T> = std::result::Result<T, error::Error>;

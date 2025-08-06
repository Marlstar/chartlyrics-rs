#![allow(clippy::needless_return)]
mod client;
#[cfg(feature = "async")]
pub use client::Client;
#[cfg(feature = "blocking")]
pub use client::BlockingClient;

pub mod models;

pub(crate) mod api;

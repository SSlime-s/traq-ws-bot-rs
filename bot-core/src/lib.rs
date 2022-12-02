//!
//!

pub mod bot;
pub use bot::builder;
pub mod events;
pub mod utils;

#[cfg(feature = "openapi")]
pub mod openapi;
#[cfg(feature = "openapi")]
#[macro_use]
extern crate serde_derive;

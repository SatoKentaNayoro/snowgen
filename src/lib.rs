//! Snowflake-RS is a library for generating unique, distributed IDs inspired by Twitter's Snowflake ID generator.

mod snowflake;
mod builder;

pub use crate::snowflake::*;
pub use crate::builder::*;
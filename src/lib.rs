//! Snowgen is a library for generating unique, distributed IDs inspired by Twitter's Snowflake ID generator.

mod builder;
mod snowgen;

pub use crate::builder::*;
pub use crate::snowgen::*;

//! Suvorov simulation core.
//!
//! Public API mirrors the C++20 slice on draft PR #2 (`feat/world-date-and-polities`):
//! a `World` owns a calendar date, named polities, named locations, and persons.
//! `advance_one_day` is the only tick. There is no map.

pub mod date;
pub mod error;
pub mod person;
pub mod world;

#[cfg(feature = "python")]
mod python;

pub use date::{biological_age, Date};
pub use error::{Error, Result};
pub use person::Person;
pub use world::World;

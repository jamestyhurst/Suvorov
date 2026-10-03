//! Suvorov simulation core.
//!
//! Public API mirrors the C++20 slice on draft PR #2 (`feat/world-date-and-polities`):
//! a `World` owns a calendar date, named polities, named locations, and persons.
//! `advance_one_day` is the only tick. Locations form an adjacency graph; ownership
//! and borders are derived (see `World::borders`). Scheduled events may carry a
//! frozen [`Effect`] list applied on the fire date. Schema-shaped records load
//! through [`world_from_records`].

pub mod date;
pub mod effect;
pub mod error;
pub mod features;
pub mod load;
pub mod person;
pub mod script;
pub mod world;

#[cfg(feature = "python")]
mod python;

pub use date::{biological_age, Date};
pub use effect::Effect;
pub use error::{Error, Result};
pub use features::{
    profile_crusader_kings_like, profile_hearts_of_iron_like, Feature, FeatureSet, GameProfile,
};
pub use load::{world_from_records, CharacterRecord, LoadedWorld, TitleRecord};
pub use person::Person;
pub use world::{Stance, World};

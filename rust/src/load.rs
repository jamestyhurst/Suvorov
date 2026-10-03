//! Schema-shaped title and character records into a `World`.
//!
//! The field names match Premyslid schema v0 so later tools can hand validated
//! dicts across. This module does not read game content and does not embed a
//! real setting. Tests use fictional Aurora records.
//!
//! Mapping
//! -------
//! - Each title becomes a named location (the title id) and a polity (the title id).
//! - A holder's allegiances are every title they hold at start. Characters who
//!   hold nothing get a required allegiance to a polity named `unlanded`.
//! - Birth and current location: the holder's first title, else `unlocated`.
//! - Characters whose death date is on or before start are loaded, then marked
//!   dead with that historical date via `World::record_death`.

use std::collections::HashMap;

use crate::date::Date;
use crate::error::Result;
use crate::person::Person;
use crate::world::World;

/// Title fields the loader reads. Extra schema keys are the caller's problem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TitleRecord {
    pub id: String,
    pub holder: Option<String>,
}

/// Character fields the loader reads. Extra schema keys are the caller's problem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterRecord {
    pub id: String,
    pub name: String,
    pub birth: String,
    pub death: Option<String>,
}

/// A loaded `World` plus the string-id maps used to build it.
pub struct LoadedWorld {
    pub world: World,
    pub person_ids: HashMap<String, u32>,
    pub polity_ids: HashMap<String, u32>,
    pub location_ids: HashMap<String, u32>,
}

/// Build a `World` from title and character records.
///
/// `titles` and `characters` are already-parsed records. Validation belongs to
/// Premyslid's tools, not to the engine.
pub fn world_from_records(
    start_date: &str,
    titles: &[TitleRecord],
    characters: &[CharacterRecord],
) -> Result<LoadedWorld> {
    let start = Date::from_iso(start_date)?;
    let mut loaded = LoadedWorld {
        world: World::new(start.year, start.month, start.day)?,
        person_ids: HashMap::new(),
        polity_ids: HashMap::new(),
        location_ids: HashMap::new(),
    };

    loaded
        .polity_ids
        .insert("unlanded".into(), loaded.world.add_polity("unlanded")?);
    loaded
        .location_ids
        .insert("unlocated".into(), loaded.world.add_location("unlocated")?);

    let mut holders: HashMap<String, Vec<String>> = HashMap::new();
    for title in titles {
        let title_id = title.id.as_str();
        loaded
            .polity_ids
            .insert(title_id.into(), loaded.world.add_polity(title_id)?);
        loaded
            .location_ids
            .insert(title_id.into(), loaded.world.add_location(title_id)?);
        if let Some(holder) = title.holder.as_deref().filter(|h| !h.is_empty()) {
            holders
                .entry(holder.to_string())
                .or_default()
                .push(title_id.to_string());
        }
    }

    for character in characters {
        let historical_death = match &character.death {
            Some(text) => {
                let death = Date::from_iso(text)?;
                if death <= start {
                    Some(death)
                } else {
                    None
                }
            }
            None => None,
        };

        let held = holders.get(&character.id).cloned().unwrap_or_default();
        let (allegiances, place) = if held.is_empty() {
            (
                vec![loaded.polity_ids["unlanded"]],
                loaded.location_ids["unlocated"],
            )
        } else {
            (
                held.iter()
                    .map(|title_id| loaded.polity_ids[title_id])
                    .collect(),
                loaded.location_ids[&held[0]],
            )
        };

        let person = Person::new(
            vec![character.name.clone()],
            allegiances,
            Date::from_iso(&character.birth)?,
            place,
            place,
        );
        let person_id = loaded.world.add_person(person)?;
        if let Some(death) = historical_death {
            loaded.world.record_death(person_id, death)?;
        }
        loaded.person_ids.insert(character.id.clone(), person_id);
    }

    Ok(loaded)
}

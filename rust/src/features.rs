//! Game-optional capabilities. The core (date, polities, locations, persons,
//! scheduled effects) is always present. A game profile turns the rest on.
//!
//! Crusader Kings and Hearts of Iron both sat on Clausewitz. Marriage, titles,
//! and inheritance are not part of every game that engine ran. Scripting is a
//! separate opt-in: the core stores a named body and can be told to fire that
//! name. It does not interpret the body.

/// A capability a game may enable. Absent means the API refuses the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Feature {
    Marriage,
    Titles,
    Inheritance,
    Scripting,
    Diplomacy,
    Forces,
    Intelligence,
    FogOfWar,
}

impl Feature {
    pub const ALL: [Feature; 8] = [
        Feature::Marriage,
        Feature::Titles,
        Feature::Inheritance,
        Feature::Scripting,
        Feature::Diplomacy,
        Feature::Forces,
        Feature::Intelligence,
        Feature::FogOfWar,
    ];

    /// Isolated suite under `rust/tests/`. The regime test fails if this file is missing.
    pub fn suite_file(self) -> &'static str {
        match self {
            Feature::Marriage => "marriage.rs",
            Feature::Titles => "titles.rs",
            Feature::Inheritance => "inheritance.rs",
            Feature::Scripting => "scripting.rs",
            Feature::Diplomacy => "diplomacy.rs",
            Feature::Forces => "forces.rs",
            Feature::Intelligence => "intelligence.rs",
            Feature::FogOfWar => "fog.rs",
        }
    }
}

/// Which optional capabilities a game has turned on.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FeatureSet {
    marriage: bool,
    titles: bool,
    inheritance: bool,
    scripting: bool,
    diplomacy: bool,
    forces: bool,
    intelligence: bool,
    fog_of_war: bool,
}

impl FeatureSet {
    pub fn none() -> Self {
        Self::default()
    }

    /// Enable one capability. Inheritance implies Titles: there is nothing to pass on otherwise.
    pub fn enable(mut self, feature: Feature) -> Self {
        match feature {
            Feature::Marriage => self.marriage = true,
            Feature::Titles => self.titles = true,
            Feature::Inheritance => {
                self.inheritance = true;
                self.titles = true;
            }
            Feature::Scripting => self.scripting = true,
            Feature::Diplomacy => self.diplomacy = true,
            Feature::Forces => self.forces = true,
            Feature::Intelligence => self.intelligence = true,
            Feature::FogOfWar => self.fog_of_war = true,
        }
        self
    }

    /// Shared grand-strategy layer: diplomacy, forces, intelligence, fog.
    pub fn grand_strategy() -> Self {
        Self::none()
            .enable(Feature::Diplomacy)
            .enable(Feature::Forces)
            .enable(Feature::Intelligence)
            .enable(Feature::FogOfWar)
    }

    pub fn contains(&self, feature: Feature) -> bool {
        match feature {
            Feature::Marriage => self.marriage,
            Feature::Titles => self.titles,
            Feature::Inheritance => self.inheritance,
            Feature::Scripting => self.scripting,
            Feature::Diplomacy => self.diplomacy,
            Feature::Forces => self.forces,
            Feature::Intelligence => self.intelligence,
            Feature::FogOfWar => self.fog_of_war,
        }
    }

    /// Dynastic game: marriage, titles, inheritance, and a script seam.
    pub fn crusader_kings_like() -> Self {
        Self::grand_strategy()
            .enable(Feature::Marriage)
            .enable(Feature::Inheritance)
            .enable(Feature::Scripting)
    }

    /// State-and-war game: grand-strategy layer and scripts. No marriage, titles, or inheritance.
    pub fn hearts_of_iron_like() -> Self {
        Self::grand_strategy().enable(Feature::Scripting)
    }
}

/// Named bundle a game app hands the engine. The engine does not pick the game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameProfile {
    pub name: &'static str,
    pub features: FeatureSet,
}

pub fn profile_crusader_kings_like() -> GameProfile {
    GameProfile {
        name: "crusader_kings_like",
        features: FeatureSet::crusader_kings_like(),
    }
}

pub fn profile_hearts_of_iron_like() -> GameProfile {
    GameProfile {
        name: "hearts_of_iron_like",
        features: FeatureSet::hearts_of_iron_like(),
    }
}

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
}

impl Feature {
    pub const ALL: [Feature; 4] = [
        Feature::Marriage,
        Feature::Titles,
        Feature::Inheritance,
        Feature::Scripting,
    ];

    /// Isolated suite under `rust/tests/`. The regime test fails if this file is missing.
    pub fn suite_file(self) -> &'static str {
        match self {
            Feature::Marriage => "marriage.rs",
            Feature::Titles => "titles.rs",
            Feature::Inheritance => "inheritance.rs",
            Feature::Scripting => "scripting.rs",
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
        }
        self
    }

    pub fn contains(&self, feature: Feature) -> bool {
        match feature {
            Feature::Marriage => self.marriage,
            Feature::Titles => self.titles,
            Feature::Inheritance => self.inheritance,
            Feature::Scripting => self.scripting,
        }
    }

    /// Dynastic game: marriage, titles, inheritance, and a script seam.
    pub fn crusader_kings_like() -> Self {
        Self::none()
            .enable(Feature::Marriage)
            .enable(Feature::Inheritance)
            .enable(Feature::Scripting)
    }

    /// State-and-war game: script seam only. No marriage, titles, or inheritance.
    pub fn hearts_of_iron_like() -> Self {
        Self::none().enable(Feature::Scripting)
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

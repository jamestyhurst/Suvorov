use crate::date::{biological_age, Date};
use crate::effect::Effect;
use crate::error::{Error, Result};
use crate::features::{Feature, FeatureSet, GameProfile};
use crate::person::Person;
use crate::script::CompiledScript;

struct Title {
    name: String,
    holder: Option<u32>,
    heir: Option<u32>,
}

struct ScheduledEvent {
    date: Date,
    name: String,
    effects: Vec<Effect>,
}

/// Deterministic world: date, named polities, named locations, persons.
pub struct World {
    current_date: Date,
    polities: Vec<String>,
    locations: Vec<String>,
    adjacency: Vec<Vec<u32>>,
    owners: Vec<Option<u32>>,
    persons: Vec<Person>,
    death_dates: Vec<Option<Date>>,
    spouses: Vec<Option<u32>>,
    titles: Vec<Title>,
    scripts: Vec<(String, CompiledScript)>,
    fired_scripts: Vec<String>,
    features: FeatureSet,
    scheduled: Vec<ScheduledEvent>,
    fired: Vec<String>,
}

impl World {
    pub fn new(year: i32, month: i32, day: i32) -> Result<Self> {
        let current_date = Date::new(year, month, day);
        if !current_date.valid() {
            return Err(Error::InvalidArgument("World date is invalid"));
        }
        Ok(Self {
            current_date,
            polities: Vec::new(),
            locations: Vec::new(),
            adjacency: Vec::new(),
            owners: Vec::new(),
            persons: Vec::new(),
            death_dates: Vec::new(),
            spouses: Vec::new(),
            titles: Vec::new(),
            scripts: Vec::new(),
            fired_scripts: Vec::new(),
            features: FeatureSet::none(),
            scheduled: Vec::new(),
            fired: Vec::new(),
        })
    }

    /// Same as `new`, with a game's optional capabilities already enabled.
    pub fn with_features(
        year: i32,
        month: i32,
        day: i32,
        features: FeatureSet,
    ) -> Result<Self> {
        let mut world = Self::new(year, month, day)?;
        world.features = features;
        Ok(world)
    }

    /// Build from a named game profile. The profile chooses features; the engine does not.
    pub fn for_profile(year: i32, month: i32, day: i32, profile: &GameProfile) -> Result<Self> {
        Self::with_features(year, month, day, profile.features.clone())
    }

    pub fn features(&self) -> &FeatureSet {
        &self.features
    }

    fn require(&self, feature: Feature) -> Result<()> {
        if self.features.contains(feature) {
            return Ok(());
        }
        let name = match feature {
            Feature::Marriage => "Marriage is not enabled for this game",
            Feature::Titles => "Titles are not enabled for this game",
            Feature::Inheritance => "Inheritance is not enabled for this game",
            Feature::Scripting => "Scripting is not enabled for this game",
        };
        Err(Error::FeatureDisabled(name))
    }

    pub fn add_polity(&mut self, name: impl Into<String>) -> Result<u32> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidArgument("Polity name cannot be empty"));
        }
        self.polities.push(name);
        Ok((self.polities.len() - 1) as u32)
    }

    pub fn polity_name(&self, polity_id: u32) -> Result<&str> {
        self.polities
            .get(polity_id as usize)
            .map(String::as_str)
            .ok_or(Error::OutOfRange("Polity id does not exist"))
    }

    pub fn add_location(&mut self, name: impl Into<String>) -> Result<u32> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidArgument("Location name cannot be empty"));
        }
        self.locations.push(name);
        self.adjacency.push(Vec::new());
        self.owners.push(None);
        Ok((self.locations.len() - 1) as u32)
    }

    pub fn location_name(&self, location_id: u32) -> Result<&str> {
        self.locations
            .get(location_id as usize)
            .map(String::as_str)
            .ok_or(Error::OutOfRange("Location id does not exist"))
    }

    /// Make two locations adjacent. Symmetric; repeating a connection is a no-op.
    pub fn connect_locations(&mut self, a: u32, b: u32) -> Result<()> {
        self.check_location(a)?;
        self.check_location(b)?;
        if a == b {
            return Err(Error::InvalidArgument("Location cannot neighbor itself"));
        }
        for (from, to) in [(a, b), (b, a)] {
            let list = &mut self.adjacency[from as usize];
            if let Err(at) = list.binary_search(&to) {
                list.insert(at, to);
            }
        }
        Ok(())
    }

    /// Neighbors of a location, in ascending id order.
    pub fn neighbors(&self, location_id: u32) -> Result<&[u32]> {
        self.check_location(location_id)?;
        Ok(&self.adjacency[location_id as usize])
    }

    pub fn set_location_owner(&mut self, location_id: u32, owner: Option<u32>) -> Result<()> {
        self.check_location(location_id)?;
        if let Some(polity_id) = owner {
            if polity_id as usize >= self.polities.len() {
                return Err(Error::OutOfRange("Polity id does not exist"));
            }
        }
        self.owners[location_id as usize] = owner;
        Ok(())
    }

    pub fn location_owner(&self, location_id: u32) -> Result<Option<u32>> {
        self.check_location(location_id)?;
        Ok(self.owners[location_id as usize])
    }

    /// Borders are derived, never stored: each `(a, b)` with `a < b` is an adjacent
    /// pair whose locations are owned by two different polities. Unowned land has none.
    pub fn borders(&self) -> Vec<(u32, u32)> {
        let mut out = Vec::new();
        for (a, neighbors) in self.adjacency.iter().enumerate() {
            for &b in neighbors.iter().filter(|&&b| b as usize > a) {
                if let (Some(x), Some(y)) = (self.owners[a], self.owners[b as usize]) {
                    if x != y {
                        out.push((a as u32, b));
                    }
                }
            }
        }
        out
    }

    fn check_location(&self, location_id: u32) -> Result<()> {
        if location_id as usize >= self.locations.len() {
            return Err(Error::OutOfRange("Location id does not exist"));
        }
        Ok(())
    }

    pub fn add_person(&mut self, person: Person) -> Result<u32> {
        if person.names.is_empty() {
            return Err(Error::InvalidArgument("Person must have at least one name"));
        }
        if person.names.iter().any(|n| n.is_empty()) {
            return Err(Error::InvalidArgument("Person name cannot be empty"));
        }
        if person.allegiances.is_empty() {
            return Err(Error::InvalidArgument(
                "Person must have at least one allegiance",
            ));
        }
        for allegiance in &person.allegiances {
            if *allegiance as usize >= self.polities.len() {
                return Err(Error::OutOfRange("Polity id does not exist"));
            }
        }
        if !person.birth_date.valid() {
            return Err(Error::InvalidArgument("Person birth date is invalid"));
        }
        if self.current_date < person.birth_date {
            return Err(Error::InvalidArgument(
                "Person birth date is after the world date",
            ));
        }
        if person.birth_location_id as usize >= self.locations.len()
            || person.current_location_id as usize >= self.locations.len()
        {
            return Err(Error::OutOfRange("Location id does not exist"));
        }
        self.persons.push(person);
        self.death_dates.push(None);
        self.spouses.push(None);
        Ok((self.persons.len() - 1) as u32)
    }

    /// Pair two living persons. Refused when Marriage is off.
    pub fn contract_marriage(&mut self, a: u32, b: u32) -> Result<()> {
        self.require(Feature::Marriage)?;
        self.require_living(a)?;
        self.require_living(b)?;
        if a == b {
            return Err(Error::InvalidArgument("A person cannot marry themselves"));
        }
        if self.spouses[a as usize].is_some() || self.spouses[b as usize].is_some() {
            return Err(Error::InvalidArgument("Person is already married"));
        }
        self.spouses[a as usize] = Some(b);
        self.spouses[b as usize] = Some(a);
        Ok(())
    }

    pub fn spouse(&self, person_id: u32) -> Result<Option<u32>> {
        self.require(Feature::Marriage)?;
        self.spouses
            .get(person_id as usize)
            .copied()
            .ok_or(Error::OutOfRange("Person id does not exist"))
    }

    /// Create a named title. Refused when Titles is off. Holder, if set, must be alive.
    pub fn create_title(&mut self, name: impl Into<String>, holder: Option<u32>) -> Result<u32> {
        self.require(Feature::Titles)?;
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidArgument("Title name cannot be empty"));
        }
        if let Some(person_id) = holder {
            self.require_living(person_id)?;
        }
        self.titles.push(Title {
            name,
            holder,
            heir: None,
        });
        Ok((self.titles.len() - 1) as u32)
    }

    pub fn title_holder(&self, title_id: u32) -> Result<Option<u32>> {
        self.require(Feature::Titles)?;
        self.titles
            .get(title_id as usize)
            .map(|title| title.holder)
            .ok_or(Error::OutOfRange("Title id does not exist"))
    }

    pub fn title_name(&self, title_id: u32) -> Result<&str> {
        self.require(Feature::Titles)?;
        self.titles
            .get(title_id as usize)
            .map(|title| title.name.as_str())
            .ok_or(Error::OutOfRange("Title id does not exist"))
    }

    /// Name who receives this title when the holder dies. Refused when Inheritance is off.
    pub fn designate_heir(&mut self, title_id: u32, heir: u32) -> Result<()> {
        self.require(Feature::Inheritance)?;
        if title_id as usize >= self.titles.len() {
            return Err(Error::OutOfRange("Title id does not exist"));
        }
        self.require_living(heir)?;
        self.titles[title_id as usize].heir = Some(heir);
        Ok(())
    }

    /// Compile a Rune body under `name`. It must define `on_fire(year, month, day)`.
    pub fn bind_script(&mut self, name: impl Into<String>, body: impl Into<String>) -> Result<()> {
        self.require(Feature::Scripting)?;
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidArgument("Script name cannot be empty"));
        }
        let compiled = CompiledScript::compile(&name, &body.into())?;
        if let Some(slot) = self.scripts.iter_mut().find(|(existing, _)| existing == &name) {
            slot.1 = compiled;
        } else {
            self.scripts.push((name, compiled));
        }
        Ok(())
    }

    pub fn script_body(&self, name: &str) -> Result<Option<&str>> {
        self.require(Feature::Scripting)?;
        Ok(self
            .scripts
            .iter()
            .find(|(existing, _)| existing == name)
            .map(|(_, script)| script.body()))
    }

    /// `name=return` for each script that fired since the last drain, in schedule order.
    pub fn drain_fired_scripts(&mut self) -> Vec<String> {
        std::mem::take(&mut self.fired_scripts)
    }

    fn require_living(&self, person_id: u32) -> Result<()> {
        if !self.is_alive(person_id)? {
            return Err(Error::InvalidArgument("Person is dead"));
        }
        Ok(())
    }

    fn transfer_titles_on_death(&mut self, person_id: u32) {
        if !self.features.contains(Feature::Inheritance) {
            return;
        }
        for title in &mut self.titles {
            if title.holder != Some(person_id) {
                continue;
            }
            let heir = title.heir.take();
            if let Some(heir_id) = heir {
                if heir_id != person_id && self.death_dates.get(heir_id as usize) == Some(&None) {
                    title.holder = Some(heir_id);
                }
            }
        }
    }

    pub fn person(&self, person_id: u32) -> Result<&Person> {
        self.persons
            .get(person_id as usize)
            .ok_or(Error::OutOfRange("Person id does not exist"))
    }

    pub fn person_age(&self, person_id: u32) -> Result<i32> {
        let person = self.person(person_id)?;
        biological_age(person.birth_date, self.current_date)
    }

    pub fn set_person_location(&mut self, person_id: u32, location_id: u32) -> Result<()> {
        if person_id as usize >= self.persons.len() {
            return Err(Error::OutOfRange("Person id does not exist"));
        }
        if location_id as usize >= self.locations.len() {
            return Err(Error::OutOfRange("Location id does not exist"));
        }
        self.persons[person_id as usize].current_location_id = location_id;
        Ok(())
    }

    /// Advance one day, apply due event effects, then queue those event names.
    pub fn advance_one_day(&mut self) {
        self.current_date.advance_one_day();
        let today = self.current_date;
        let (due, later): (Vec<_>, Vec<_>) = self
            .scheduled
            .drain(..)
            .partition(|event| event.date == today);
        self.scheduled = later;
        for event in due {
            for effect in event.effects {
                self.apply_effect(effect);
            }
            self.fired.push(event.name);
        }
    }

    /// Queue a named event to fire when the world reaches `date` (strictly in the future).
    pub fn schedule_event(&mut self, date: Date, name: impl Into<String>) -> Result<()> {
        self.schedule_event_with_effects(date, name, Vec::new())
    }

    /// Queue a named event that applies `effects` when it fires.
    pub fn schedule_event_with_effects(
        &mut self,
        date: Date,
        name: impl Into<String>,
        effects: Vec<Effect>,
    ) -> Result<()> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::InvalidArgument("Event name cannot be empty"));
        }
        if !date.valid() {
            return Err(Error::InvalidArgument("Event date is invalid"));
        }
        if date <= self.current_date {
            return Err(Error::InvalidArgument(
                "Event date must be after the world date",
            ));
        }
        for effect in &effects {
            self.check_effect(effect)?;
        }
        self.scheduled.push(ScheduledEvent {
            date,
            name,
            effects,
        });
        Ok(())
    }

    fn check_effect(&self, effect: &Effect) -> Result<()> {
        match effect {
            Effect::KillPerson(person_id) => {
                if *person_id as usize >= self.persons.len() {
                    return Err(Error::OutOfRange("Person id does not exist"));
                }
                Ok(())
            }
            Effect::SetLocationOwner { location_id, owner } => {
                self.check_location(*location_id)?;
                if let Some(polity_id) = owner {
                    if *polity_id as usize >= self.polities.len() {
                        return Err(Error::OutOfRange("Polity id does not exist"));
                    }
                }
                Ok(())
            }
            Effect::RunScript(name) => {
                self.require(Feature::Scripting)?;
                if name.is_empty() {
                    return Err(Error::InvalidArgument("Script name cannot be empty"));
                }
                if !self.scripts.iter().any(|(existing, _)| existing == name) {
                    return Err(Error::InvalidArgument("Script is not bound"));
                }
                Ok(())
            }
        }
    }

    fn apply_effect(&mut self, effect: Effect) {
        match effect {
            Effect::KillPerson(person_id) => {
                let _ = self.kill_person(person_id);
            }
            Effect::SetLocationOwner { location_id, owner } => {
                let _ = self.set_location_owner(location_id, owner);
            }
            Effect::RunScript(name) => {
                if let Some((_, script)) = self.scripts.iter().find(|(existing, _)| existing == &name) {
                    let today = self.current_date;
                    match script.call_on_fire(today.year, today.month, today.day) {
                        Ok(value) => self.fired_scripts.push(format!("{name}={value}")),
                        Err(err) => self.fired_scripts.push(format!("{name}!{err}")),
                    }
                }
            }
        }
    }

    /// Take the events that fired since the last drain, in the order they were scheduled.
    pub fn drain_fired_events(&mut self) -> Vec<String> {
        std::mem::take(&mut self.fired)
    }

    /// Record that `person_id` died on the current world date.
    pub fn kill_person(&mut self, person_id: u32) -> Result<()> {
        self.record_death(person_id, self.current_date)
    }

    /// Record a death on `date` (at or before the world date, at or after birth).
    pub fn record_death(&mut self, person_id: u32, date: Date) -> Result<()> {
        let birth_date = self.person(person_id)?.birth_date;
        if !date.valid() {
            return Err(Error::InvalidArgument("Death date is invalid"));
        }
        if date < birth_date {
            return Err(Error::InvalidArgument("Death date is before birth date"));
        }
        if date > self.current_date {
            return Err(Error::InvalidArgument("Death date is after the world date"));
        }
        let slot = &mut self.death_dates[person_id as usize];
        if slot.is_some() {
            return Err(Error::InvalidArgument("Person is already dead"));
        }
        *slot = Some(date);
        self.transfer_titles_on_death(person_id);
        if self.features.contains(Feature::Marriage) {
            if let Some(spouse_id) = self.spouses[person_id as usize].take() {
                if let Some(back) = self.spouses.get_mut(spouse_id as usize) {
                    if *back == Some(person_id) {
                        *back = None;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn person_death_date(&self, person_id: u32) -> Result<Option<Date>> {
        self.death_dates
            .get(person_id as usize)
            .copied()
            .ok_or(Error::OutOfRange("Person id does not exist"))
    }

    pub fn is_alive(&self, person_id: u32) -> Result<bool> {
        Ok(self.person_death_date(person_id)?.is_none())
    }

    pub fn living_person_count(&self) -> usize {
        self.death_dates.iter().filter(|d| d.is_none()).count()
    }

    pub fn date(&self) -> Date {
        self.current_date
    }

    pub fn person_count(&self) -> usize {
        self.persons.len()
    }
}

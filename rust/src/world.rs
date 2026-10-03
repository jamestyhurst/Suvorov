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

struct Office {
    polity_id: u32,
    name: String,
    person_id: u32,
}

struct Force {
    owner: u32,
    kind: String,
    location_id: u32,
    strength: u32,
}

struct Operation {
    actor: u32,
    target: u32,
    kind: String,
    revealed: bool,
}

/// Shared stance between two polities. Opinion is separate and directed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    Peace,
    War,
    Truce,
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
    offices: Vec<Office>,
    stances: Vec<(u32, u32, Stance)>,
    opinions: Vec<(u32, u32, i32)>,
    forces: Vec<Force>,
    operations: Vec<Operation>,
    revealed: Vec<(u32, u32)>,
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
            offices: Vec::new(),
            stances: Vec::new(),
            opinions: Vec::new(),
            forces: Vec::new(),
            operations: Vec::new(),
            revealed: Vec::new(),
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
            Feature::Diplomacy => "Diplomacy is not enabled for this game",
            Feature::Forces => "Forces are not enabled for this game",
            Feature::Intelligence => "Intelligence is not enabled for this game",
            Feature::FogOfWar => "Fog of war is not enabled for this game",
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
            Effect::MovePerson { person_id, location_id } => {
                if *person_id as usize >= self.persons.len() {
                    return Err(Error::OutOfRange("Person id does not exist"));
                }
                self.check_location(*location_id)
            }
            Effect::SetAllegiance { person_id, polity_id } => {
                if *person_id as usize >= self.persons.len() {
                    return Err(Error::OutOfRange("Person id does not exist"));
                }
                if *polity_id as usize >= self.polities.len() {
                    return Err(Error::OutOfRange("Polity id does not exist"));
                }
                Ok(())
            }
        }
    }

    /// Replace a living person's allegiances with one polity.
    pub fn set_person_allegiance(&mut self, person_id: u32, polity_id: u32) -> Result<()> {
        self.require_living(person_id)?;
        if polity_id as usize >= self.polities.len() {
            return Err(Error::OutOfRange("Polity id does not exist"));
        }
        self.persons[person_id as usize].allegiances = vec![polity_id];
        Ok(())
    }

    fn apply_effect(&mut self, effect: Effect) {
        match effect {
            Effect::KillPerson(person_id) => {
                let _ = self.kill_person(person_id);
            }
            Effect::SetLocationOwner { location_id, owner } => {
                let _ = self.set_location_owner(location_id, owner);
            }
            Effect::RunScript(name) => self.apply_script(&name),
            Effect::MovePerson { person_id, location_id } => {
                let _ = self.set_person_location(person_id, location_id);
            }
            Effect::SetAllegiance { person_id, polity_id } => {
                let _ = self.set_person_allegiance(person_id, polity_id);
            }
        }
    }

    fn apply_script(&mut self, name: &str) {
        let Some((_, script)) = self.scripts.iter().find(|(existing, _)| existing == name) else {
            return;
        };
        let today = self.current_date;
        let view = crate::script::ScriptView {
            year: today.year,
            month: today.month,
            day: today.day,
            marriage_enabled: self.features.contains(Feature::Marriage),
            titles_enabled: self.features.contains(Feature::Titles),
            inheritance_enabled: self.features.contains(Feature::Inheritance),
            person_count: self.persons.len() as u32,
            location_count: self.locations.len() as u32,
            polity_count: self.polities.len() as u32,
            title_count: self.titles.len() as u32,
            names: self.persons.iter().map(|person| person.names[0].clone()).collect(),
            alive: self.death_dates.iter().map(|date| date.is_none()).collect(),
        };
        match script.call_on_fire(view) {
            Ok((value, asks)) => {
                self.fired_scripts.push(format!("{name}={value}"));
                self.apply_asks(asks);
            }
            Err(err) => self.fired_scripts.push(format!("{name}!{err}")),
        }
    }

    fn apply_asks(&mut self, asks: Vec<crate::script::ScriptAsk>) {
        for ask in asks {
            match ask {
                crate::script::ScriptAsk::Marry(a, b) => {
                    let _ = self.contract_marriage(a, b);
                }
                crate::script::ScriptAsk::GrantTitle { name, holder } => {
                    let _ = self.create_title(name, Some(holder));
                }
                crate::script::ScriptAsk::DesignateHeir { title_id, heir } => {
                    let _ = self.designate_heir(title_id, heir);
                }
                crate::script::ScriptAsk::MovePerson { person_id, location_id } => {
                    let _ = self.set_person_location(person_id, location_id);
                }
                crate::script::ScriptAsk::Kill(person_id) => {
                    let _ = self.kill_person(person_id);
                }
                crate::script::ScriptAsk::SetAllegiance { person_id, polity_id } => {
                    let _ = self.set_person_allegiance(person_id, polity_id);
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

    /// Seat a living member of this polity in a named office. Core: every entity can have a ruler.
    pub fn appoint(&mut self, polity_id: u32, office: impl Into<String>, person_id: u32) -> Result<()> {
        self.polity_name(polity_id)?;
        let office = office.into();
        if office.is_empty() {
            return Err(Error::InvalidArgument("Office name cannot be empty"));
        }
        self.require_living(person_id)?;
        if !self.person(person_id)?.allegiances.contains(&polity_id) {
            return Err(Error::InvalidArgument("Person is not a member of that polity"));
        }
        if let Some(slot) = self.offices.iter_mut().find(|seat| seat.polity_id == polity_id && seat.name == office) {
            slot.person_id = person_id;
        } else {
            self.offices.push(Office { polity_id, name: office, person_id });
        }
        Ok(())
    }

    pub fn office_holder(&self, polity_id: u32, office: &str) -> Result<Option<u32>> {
        self.polity_name(polity_id)?;
        Ok(self.offices.iter().find(|seat| seat.polity_id == polity_id && seat.name == office).map(|seat| seat.person_id))
    }

    pub fn set_stance(&mut self, a: u32, b: u32, stance: Stance) -> Result<()> {
        self.require(Feature::Diplomacy)?;
        self.polity_name(a)?;
        self.polity_name(b)?;
        if a == b {
            return Err(Error::InvalidArgument("A polity has no stance toward itself"));
        }
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        if let Some(slot) = self.stances.iter_mut().find(|(x, y, _)| *x == lo && *y == hi) {
            slot.2 = stance;
        } else {
            self.stances.push((lo, hi, stance));
        }
        Ok(())
    }

    pub fn stance(&self, a: u32, b: u32) -> Result<Stance> {
        self.require(Feature::Diplomacy)?;
        self.polity_name(a)?;
        self.polity_name(b)?;
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        Ok(self.stances.iter().find(|(x, y, _)| *x == lo && *y == hi).map(|(_, _, stance)| *stance).unwrap_or(Stance::Peace))
    }

    pub fn set_opinion(&mut self, from: u32, toward: u32, value: i32) -> Result<()> {
        self.require(Feature::Diplomacy)?;
        self.polity_name(from)?;
        self.polity_name(toward)?;
        if from == toward {
            return Err(Error::InvalidArgument("A polity has no opinion of itself"));
        }
        if let Some(slot) = self.opinions.iter_mut().find(|(a, b, _)| *a == from && *b == toward) {
            slot.2 = value;
        } else {
            self.opinions.push((from, toward, value));
        }
        Ok(())
    }

    pub fn opinion(&self, from: u32, toward: u32) -> Result<i32> {
        self.require(Feature::Diplomacy)?;
        self.polity_name(from)?;
        self.polity_name(toward)?;
        Ok(self.opinions.iter().find(|(a, b, _)| *a == from && *b == toward).map(|(_, _, value)| *value).unwrap_or(0))
    }

    /// A military force. `kind` is the game's word (army, fleet, levy); the engine does not close the list.
    pub fn raise_force(&mut self, owner: u32, kind: impl Into<String>, location_id: u32, strength: u32) -> Result<u32> {
        self.require(Feature::Forces)?;
        self.polity_name(owner)?;
        self.check_location(location_id)?;
        let kind = kind.into();
        if kind.is_empty() {
            return Err(Error::InvalidArgument("Force kind cannot be empty"));
        }
        self.forces.push(Force { owner, kind, location_id, strength });
        Ok((self.forces.len() - 1) as u32)
    }

    pub fn force_kind(&self, force_id: u32) -> Result<&str> {
        self.require(Feature::Forces)?;
        self.forces.get(force_id as usize).map(|force| force.kind.as_str()).ok_or(Error::OutOfRange("Force id does not exist"))
    }

    pub fn force_strength(&self, force_id: u32) -> Result<u32> {
        self.require(Feature::Forces)?;
        self.forces.get(force_id as usize).map(|force| force.strength).ok_or(Error::OutOfRange("Force id does not exist"))
    }

    pub fn post_operation(&mut self, actor: u32, target: u32, kind: impl Into<String>) -> Result<u32> {
        self.require(Feature::Intelligence)?;
        self.polity_name(actor)?;
        self.polity_name(target)?;
        let kind = kind.into();
        if kind.is_empty() {
            return Err(Error::InvalidArgument("Operation kind cannot be empty"));
        }
        self.operations.push(Operation { actor, target, kind, revealed: false });
        Ok((self.operations.len() - 1) as u32)
    }

    pub fn reveal_operation(&mut self, operation_id: u32) -> Result<()> {
        self.require(Feature::Intelligence)?;
        let op = self.operations.get_mut(operation_id as usize).ok_or(Error::OutOfRange("Operation id does not exist"))?;
        op.revealed = true;
        Ok(())
    }

    pub fn operation_revealed(&self, operation_id: u32) -> Result<bool> {
        self.require(Feature::Intelligence)?;
        self.operations.get(operation_id as usize).map(|op| op.revealed).ok_or(Error::OutOfRange("Operation id does not exist"))
    }

    pub fn reveal_location(&mut self, polity_id: u32, location_id: u32) -> Result<()> {
        self.require(Feature::FogOfWar)?;
        self.polity_name(polity_id)?;
        self.check_location(location_id)?;
        if !self.revealed.contains(&(polity_id, location_id)) {
            self.revealed.push((polity_id, location_id));
        }
        Ok(())
    }

    /// Fog off: everything is visible. Fog on: owned land and revealed land only.
    pub fn can_see(&self, polity_id: u32, location_id: u32) -> Result<bool> {
        self.polity_name(polity_id)?;
        self.check_location(location_id)?;
        if !self.features.contains(Feature::FogOfWar) {
            return Ok(true);
        }
        if self.owners[location_id as usize] == Some(polity_id) {
            return Ok(true);
        }
        Ok(self.revealed.contains(&(polity_id, location_id)))
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

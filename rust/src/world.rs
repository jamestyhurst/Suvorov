use crate::date::{biological_age, Date};
use crate::effect::Effect;
use crate::error::{Error, Result};
use crate::person::Person;

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
            scheduled: Vec::new(),
            fired: Vec::new(),
        })
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
        Ok((self.persons.len() - 1) as u32)
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
        }
    }

    /// Take the events that fired since the last drain, in the order they were scheduled.
    pub fn drain_fired_events(&mut self) -> Vec<String> {
        std::mem::take(&mut self.fired)
    }

    pub fn kill_person(&mut self, person_id: u32) -> Result<()> {
        let slot = self
            .death_dates
            .get_mut(person_id as usize)
            .ok_or(Error::OutOfRange("Person id does not exist"))?;
        if slot.is_some() {
            return Err(Error::InvalidArgument("Person is already dead"));
        }
        *slot = Some(self.current_date);
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

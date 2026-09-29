use crate::date::{biological_age, Date};
use crate::error::{Error, Result};
use crate::person::Person;

/// Deterministic world: date, named polities, named locations, persons.
pub struct World {
    current_date: Date,
    polities: Vec<String>,
    locations: Vec<String>,
    persons: Vec<Person>,
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
            persons: Vec::new(),
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
        Ok((self.locations.len() - 1) as u32)
    }

    pub fn location_name(&self, location_id: u32) -> Result<&str> {
        self.locations
            .get(location_id as usize)
            .map(String::as_str)
            .ok_or(Error::OutOfRange("Location id does not exist"))
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

    pub fn advance_one_day(&mut self) {
        self.current_date.advance_one_day();
    }

    pub fn date(&self) -> Date {
        self.current_date
    }

    pub fn person_count(&self) -> usize {
        self.persons.len()
    }
}

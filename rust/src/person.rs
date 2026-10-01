use crate::date::Date;

/// Public person record. Fields match `include/suvorov/core/person.hpp` on PR #2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Person {
    pub names: Vec<String>,
    pub allegiances: Vec<u32>,
    pub birth_date: Date,
    pub birth_location_id: u32,
    pub current_location_id: u32,
}

impl Person {
    pub fn new(
        names: Vec<String>,
        allegiances: Vec<u32>,
        birth_date: Date,
        birth_location_id: u32,
        current_location_id: u32,
    ) -> Self {
        Self {
            names,
            allegiances,
            birth_date,
            birth_location_id,
            current_location_id,
        }
    }
}

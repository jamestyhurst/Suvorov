/// A frozen world change applied when a scheduled event fires.
///
/// Effects run inside `World::advance_one_day` after the calendar moves, before
/// the event name is queued for `drain_fired_events`. The tick does not run
/// game scripts; callers choose the effects when they schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    KillPerson(u32),
    SetLocationOwner {
        location_id: u32,
        owner: Option<u32>,
    },
    /// Name of a bound Rune script. The tick calls `on_fire` and records `name=return`.
    RunScript(String),
    MovePerson { person_id: u32, location_id: u32 },
    SetAllegiance { person_id: u32, polity_id: u32 },
}

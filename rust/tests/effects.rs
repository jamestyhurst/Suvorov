//! Core effects that every game has: a person can move, and allegiance can change.

use suvorov_core::{Date, Effect, Person, World};

fn world() -> World {
    let mut world = World::new(1001, 1, 1).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world.add_location("Birchford").unwrap();
    world
        .add_person(Person::new(vec!["Alder".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
    world
}

#[test]
fn a_scheduled_move_changes_the_persons_location() {
    let mut world = world();
    world
        .schedule_event_with_effects(
            Date::new(1001, 1, 2),
            "march",
            vec![Effect::MovePerson { person_id: 0, location_id: 1 }],
        )
        .unwrap();
    world.advance_one_day();
    assert_eq!(world.person(0).unwrap().current_location_id, 1);
}

#[test]
fn a_scheduled_allegiance_replaces_the_old_one() {
    let mut world = world();
    world
        .schedule_event_with_effects(
            Date::new(1001, 1, 2),
            "oath",
            vec![Effect::SetAllegiance { person_id: 0, polity_id: 1 }],
        )
        .unwrap();
    world.advance_one_day();
    assert_eq!(world.person(0).unwrap().allegiances, vec![1]);
}

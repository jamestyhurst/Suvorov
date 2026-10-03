use suvorov_core::{Date, Effect, Error, Person, World};

fn world_with_person() -> (World, u32) {
    let mut world = World::new(2024, 1, 1).unwrap();
    let polity = world.add_polity("Aurora").unwrap();
    let vale = world.add_location("Amber Vale").unwrap();
    let id = world
        .add_person(Person::new(
            vec!["Calen".into()],
            vec![polity],
            Date::new(1990, 6, 1),
            vale,
            vale,
        ))
        .unwrap();
    (world, id)
}

#[test]
fn people_start_alive_and_death_is_recorded_once() {
    let (mut world, id) = world_with_person();
    assert!(world.is_alive(id).unwrap());
    assert_eq!(world.living_person_count(), 1);
    assert_eq!(world.person_death_date(id).unwrap(), None);

    world.kill_person(id).unwrap();
    assert!(!world.is_alive(id).unwrap());
    assert_eq!(
        world.person_death_date(id).unwrap(),
        Some(Date::new(2024, 1, 1))
    );
    assert_eq!(world.living_person_count(), 0);
    // The record stays: dead people are still people.
    assert_eq!(world.person_count(), 1);

    assert!(matches!(
        world.kill_person(id),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(world.kill_person(9), Err(Error::OutOfRange(_))));
}

#[test]
fn scheduled_events_fire_on_their_date_in_schedule_order() {
    let mut world = World::new(2024, 12, 30).unwrap();
    world
        .schedule_event(Date::new(2024, 12, 31), "first")
        .unwrap();
    world
        .schedule_event(Date::new(2025, 1, 1), "new year")
        .unwrap();
    world
        .schedule_event(Date::new(2024, 12, 31), "second")
        .unwrap();

    assert!(world.drain_fired_events().is_empty());

    world.advance_one_day(); // 12-31
    assert_eq!(world.drain_fired_events(), vec!["first", "second"]);
    // Draining empties the queue.
    assert!(world.drain_fired_events().is_empty());

    world.advance_one_day(); // 01-01
    assert_eq!(world.drain_fired_events(), vec!["new year"]);
}

#[test]
fn scheduling_rejects_past_today_invalid_dates_and_empty_names() {
    let mut world = World::new(2024, 1, 1).unwrap();
    assert!(matches!(
        world.schedule_event(Date::new(2023, 12, 31), "past"),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(
        world.schedule_event(Date::new(2024, 2, 30), "bad"),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(
        world.schedule_event(Date::new(2024, 1, 2), ""),
        Err(Error::InvalidArgument(_))
    ));
    // Today has already happened; events must be strictly in the future.
    assert!(matches!(
        world.schedule_event(Date::new(2024, 1, 1), "today"),
        Err(Error::InvalidArgument(_))
    ));
}

#[test]
fn scheduled_kill_applies_when_the_event_fires() {
    let (mut world, id) = world_with_person();
    world
        .schedule_event_with_effects(
            Date::new(2024, 1, 2),
            "calen dies",
            vec![Effect::KillPerson(id)],
        )
        .unwrap();

    assert!(world.is_alive(id).unwrap());
    world.advance_one_day();
    assert!(!world.is_alive(id).unwrap());
    assert_eq!(
        world.person_death_date(id).unwrap(),
        Some(Date::new(2024, 1, 2))
    );
    assert_eq!(world.drain_fired_events(), vec!["calen dies"]);
}

#[test]
fn scheduling_a_kill_rejects_an_unknown_person() {
    let (mut world, _id) = world_with_person();
    assert!(matches!(
        world.schedule_event_with_effects(
            Date::new(2024, 1, 2),
            "ghost",
            vec![Effect::KillPerson(99)],
        ),
        Err(Error::OutOfRange(_))
    ));
}

#[test]
fn scheduled_owner_change_moves_the_derived_border() {
    let mut world = World::new(2024, 1, 1).unwrap();
    let aurora = world.add_polity("Aurora").unwrap();
    let helia = world.add_polity("Helia").unwrap();
    let vale = world.add_location("Amber Vale").unwrap();
    let ridge = world.add_location("Glass Ridge").unwrap();
    world.connect_locations(vale, ridge).unwrap();
    world.set_location_owner(vale, Some(aurora)).unwrap();
    world.set_location_owner(ridge, Some(aurora)).unwrap();
    assert!(world.borders().is_empty());

    world
        .schedule_event_with_effects(
            Date::new(2024, 1, 2),
            "ridge ceded",
            vec![Effect::SetLocationOwner {
                location_id: ridge,
                owner: Some(helia),
            }],
        )
        .unwrap();

    world.advance_one_day();
    assert_eq!(world.location_owner(ridge).unwrap(), Some(helia));
    assert_eq!(world.borders(), vec![(vale, ridge)]);
    assert_eq!(world.drain_fired_events(), vec!["ridge ceded"]);
}

#[test]
fn scheduled_kill_of_an_already_dead_person_still_fires() {
    let (mut world, id) = world_with_person();
    world
        .schedule_event_with_effects(
            Date::new(2024, 1, 2),
            "calen dies",
            vec![Effect::KillPerson(id)],
        )
        .unwrap();
    world.kill_person(id).unwrap();
    let death = world.person_death_date(id).unwrap();

    world.advance_one_day();
    assert_eq!(world.person_death_date(id).unwrap(), death);
    assert_eq!(world.drain_fired_events(), vec!["calen dies"]);
}

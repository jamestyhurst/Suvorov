use suvorov_core::{Date, Error, Person, World};

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

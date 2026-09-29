use suvorov_core::{biological_age, Date, Error, Person, World};

fn aurora_world() -> (World, u32, u32, u32, u32) {
    let mut world = World::new(2024, 1, 1).unwrap();
    let polity = world.add_polity("Aurora").unwrap();
    let second = world.add_polity("Helia").unwrap();
    let vale = world.add_location("Amber Vale").unwrap();
    let ridge = world.add_location("Glass Ridge").unwrap();
    (world, polity, second, vale, ridge)
}

#[test]
fn named_polities_and_locations() {
    let (world, polity, second, vale, ridge) = aurora_world();
    assert_eq!(world.polity_name(polity).unwrap(), "Aurora");
    assert_ne!(second, polity);
    assert_eq!(world.polity_name(second).unwrap(), "Helia");
    assert_eq!(world.location_name(vale).unwrap(), "Amber Vale");
    assert_ne!(ridge, vale);
}

#[test]
fn person_record_and_derived_age() {
    let (mut world, polity, second, vale, ridge) = aurora_world();
    let calen = Person::new(
        vec!["Calen".into(), "of Vale".into()],
        vec![polity, second],
        Date::new(2000, 3, 1),
        vale,
        vale,
    );
    let person_id = world.add_person(calen).unwrap();
    let stored = world.person(person_id).unwrap();
    assert_eq!(stored.names.len(), 2);
    assert_eq!(stored.names[0], "Calen");
    assert_eq!(stored.names[1], "of Vale");
    assert_eq!(stored.allegiances, vec![polity, second]);
    assert_eq!(stored.birth_date, Date::new(2000, 3, 1));
    assert_eq!(stored.birth_location_id, vale);
    assert_eq!(stored.current_location_id, vale);
    assert_eq!(world.person_age(person_id).unwrap(), 23);

    let mira = Person::new(
        vec!["Mira".into()],
        vec![polity],
        Date::new(2000, 3, 1),
        vale,
        ridge,
    );
    let mira_id = world.add_person(mira).unwrap();
    assert_ne!(mira_id, person_id);
    assert_eq!(world.person(mira_id).unwrap().current_location_id, ridge);

    world.set_person_location(person_id, ridge).unwrap();
    assert_eq!(world.person(person_id).unwrap().current_location_id, ridge);
    assert_eq!(world.person(person_id).unwrap().birth_location_id, vale);

    world.advance_one_day();
    let date = world.date();
    assert_eq!(date, Date::new(2024, 1, 2));
    assert_eq!(world.person_age(person_id).unwrap(), 23);
}

#[test]
fn birthday_on_leap_day_rollover() {
    let mut world = World::new(2024, 2, 29).unwrap();
    let polity = world.add_polity("Aurora").unwrap();
    let place = world.add_location("Amber Vale").unwrap();
    let id = world
        .add_person(Person::new(
            vec!["Calen".into()],
            vec![polity],
            Date::new(2000, 3, 1),
            place,
            place,
        ))
        .unwrap();
    assert_eq!(world.person_age(id).unwrap(), 23);
    world.advance_one_day();
    assert_eq!(world.date(), Date::new(2024, 3, 1));
    assert_eq!(world.person_age(id).unwrap(), 24);
}

#[test]
fn infant_born_today_is_age_zero() {
    let mut world = World::new(2024, 6, 15).unwrap();
    let polity = world.add_polity("Helia").unwrap();
    let place = world.add_location("Glass Ridge").unwrap();
    let id = world
        .add_person(Person::new(
            vec!["Ryn".into()],
            vec![polity],
            Date::new(2024, 6, 15),
            place,
            place,
        ))
        .unwrap();
    assert_eq!(world.person_age(id).unwrap(), 0);
}

#[test]
fn calendar_edges() {
    let mut end = World::new(2024, 12, 31).unwrap();
    end.advance_one_day();
    assert_eq!(end.date(), Date::new(2025, 1, 1));

    let mut leap = World::new(2024, 2, 28).unwrap();
    leap.advance_one_day();
    assert_eq!(leap.date(), Date::new(2024, 2, 29));
    leap.advance_one_day();
    assert_eq!(leap.date(), Date::new(2024, 3, 1));

    let mut non_leap = World::new(2023, 2, 28).unwrap();
    non_leap.advance_one_day();
    assert_eq!(non_leap.date(), Date::new(2023, 3, 1));
}

#[test]
fn biological_age_leap_birthday() {
    assert_eq!(
        biological_age(Date::new(2000, 2, 29), Date::new(2023, 2, 28)).unwrap(),
        22
    );
    assert_eq!(
        biological_age(Date::new(2000, 2, 29), Date::new(2023, 3, 1)).unwrap(),
        23
    );
}

#[test]
fn rejects_invalid_world_date() {
    assert!(matches!(
        World::new(2024, 2, 30),
        Err(Error::InvalidArgument(_))
    ));
}

#[test]
fn rejects_empty_and_unknown_ids() {
    let (mut world, _, _, vale, _) = aurora_world();
    assert!(matches!(
        world.add_polity(""),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(world.polity_name(99), Err(Error::OutOfRange(_))));
    assert!(matches!(
        world.add_location(""),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(world.location_name(99), Err(Error::OutOfRange(_))));
    assert!(matches!(world.person(99), Err(Error::OutOfRange(_))));
    assert!(matches!(
        world.set_person_location(99, vale),
        Err(Error::OutOfRange(_))
    ));
}

#[test]
fn rejects_bad_persons() {
    let (mut world, polity, _, vale, _) = aurora_world();
    let good = Person::new(
        vec!["Mira".into()],
        vec![polity],
        Date::new(2000, 3, 1),
        vale,
        vale,
    );
    let id = world.add_person(good.clone()).unwrap();

    let mut no_names = good.clone();
    no_names.names.clear();
    assert!(matches!(
        world.add_person(no_names),
        Err(Error::InvalidArgument(_))
    ));

    let mut empty_name = good.clone();
    empty_name.names = vec!["".into()];
    assert!(matches!(
        world.add_person(empty_name),
        Err(Error::InvalidArgument(_))
    ));

    let mut no_allegiance = good.clone();
    no_allegiance.allegiances.clear();
    assert!(matches!(
        world.add_person(no_allegiance),
        Err(Error::InvalidArgument(_))
    ));

    let mut bad_allegiance = good.clone();
    bad_allegiance.allegiances = vec![99];
    assert!(matches!(
        world.add_person(bad_allegiance),
        Err(Error::OutOfRange(_))
    ));

    let mut bad_birth = good.clone();
    bad_birth.birth_date = Date::new(2024, 2, 30);
    assert!(matches!(
        world.add_person(bad_birth),
        Err(Error::InvalidArgument(_))
    ));

    let mut future = good.clone();
    future.birth_date = Date::new(2025, 1, 1);
    assert!(matches!(
        world.add_person(future),
        Err(Error::InvalidArgument(_))
    ));

    let mut bad_place = good;
    bad_place.birth_location_id = 99;
    assert!(matches!(
        world.add_person(bad_place),
        Err(Error::OutOfRange(_))
    ));

    assert!(matches!(
        world.set_person_location(id, 99),
        Err(Error::OutOfRange(_))
    ));
}

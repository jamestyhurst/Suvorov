//! Isolated suite for Inheritance. Enabling it also enables Titles.

use suvorov_core::{Feature, FeatureSet, Date, Person, World};

fn world() -> World {
    let mut world =
        World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Inheritance)).unwrap();
    assert!(world.features().contains(Feature::Titles));
    assert!(!world.features().contains(Feature::Marriage));
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world
        .add_person(Person::new(vec!["Alder".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
    world
        .add_person(Person::new(vec!["Birch".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
    world
}

#[test]
fn a_designated_heir_receives_the_title() {
    let mut world = world();
    let title = world.create_title("Northfold", Some(0)).unwrap();
    world.designate_heir(title, 1).unwrap();
    world.kill_person(0).unwrap();
    assert_eq!(world.title_holder(title).unwrap(), Some(1));
}

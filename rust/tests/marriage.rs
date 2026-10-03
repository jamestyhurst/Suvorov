//! Isolated suite for Marriage. Titles stay off.

use suvorov_core::{Error, Feature, FeatureSet, Date, Person, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Marriage)).unwrap();
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
fn marriage_pairs_two_living_persons() {
    let mut world = world();
    world.contract_marriage(0, 1).unwrap();
    assert_eq!(world.spouse(0).unwrap(), Some(1));
}

#[test]
fn marriage_mode_still_refuses_titles() {
    let mut world = world();
    assert!(matches!(
        world.create_title("Northfold", Some(0)),
        Err(Error::FeatureDisabled(_))
    ));
}

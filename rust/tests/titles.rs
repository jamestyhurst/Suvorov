//! Isolated suite for Titles. Inheritance stays off, so death does not move a title.

use suvorov_core::{Error, Feature, FeatureSet, Date, Person, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Titles)).unwrap();
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
fn a_title_can_be_granted_without_inheritance() {
    let mut world = world();
    let title = world.create_title("Northfold", Some(0)).unwrap();
    assert_eq!(world.title_name(title).unwrap(), "Northfold");
    assert_eq!(world.title_holder(title).unwrap(), Some(0));
}

#[test]
fn titles_mode_refuses_an_heir_and_keeps_the_dead_holder() {
    let mut world = world();
    let title = world.create_title("Northfold", Some(0)).unwrap();
    assert!(matches!(
        world.designate_heir(title, 1),
        Err(Error::FeatureDisabled(_))
    ));
    world.kill_person(0).unwrap();
    assert_eq!(world.title_holder(title).unwrap(), Some(0));
}

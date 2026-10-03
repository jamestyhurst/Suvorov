//! Isolated suite for Forces. Kind is a game word, not a closed engine list.

use suvorov_core::{Error, Feature, FeatureSet, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Forces)).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world
}

#[test]
fn two_kinds_of_force_can_stand_in_one_place() {
    let mut world = world();
    let army = world.raise_force(0, "army", 0, 100).unwrap();
    let fleet = world.raise_force(0, "fleet", 0, 12).unwrap();
    assert_eq!(world.force_kind(army).unwrap(), "army");
    assert_eq!(world.force_kind(fleet).unwrap(), "fleet");
    assert_eq!(world.force_strength(fleet).unwrap(), 12);
}

#[test]
fn forces_off_refuses_a_raise() {
    let mut world = World::new(1001, 1, 1).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    assert!(matches!(world.raise_force(0, "army", 0, 1), Err(Error::FeatureDisabled(_))));
}

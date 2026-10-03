//! Isolated suite for Intelligence. An operation starts hidden.

use suvorov_core::{Error, Feature, FeatureSet, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Intelligence)).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    world
}

#[test]
fn an_operation_is_hidden_until_revealed() {
    let mut world = world();
    let op = world.post_operation(0, 1, "scout").unwrap();
    assert!(!world.operation_revealed(op).unwrap());
    world.reveal_operation(op).unwrap();
    assert!(world.operation_revealed(op).unwrap());
}

#[test]
fn intelligence_off_refuses_an_operation() {
    let mut world = World::new(1001, 1, 1).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    assert!(matches!(world.post_operation(0, 1, "scout"), Err(Error::FeatureDisabled(_))));
}

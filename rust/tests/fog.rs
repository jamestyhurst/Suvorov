//! Isolated suite for fog of war. Off means the map is visible.

use suvorov_core::{Error, Feature, FeatureSet, World};

#[test]
fn fog_hides_foreign_land_until_it_is_revealed() {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::FogOfWar)).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world.add_location("Birchford").unwrap();
    world.set_location_owner(0, Some(0)).unwrap();
    world.set_location_owner(1, Some(1)).unwrap();
    assert!(world.can_see(0, 0).unwrap());
    assert!(!world.can_see(0, 1).unwrap());
    world.reveal_location(0, 1).unwrap();
    assert!(world.can_see(0, 1).unwrap());
}

#[test]
fn fog_off_sees_every_location() {
    let mut world = World::new(1001, 1, 1).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    assert!(world.can_see(0, 0).unwrap());
    assert!(matches!(world.reveal_location(0, 0), Err(Error::FeatureDisabled(_))));
}

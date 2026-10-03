//! Isolated suite for Diplomacy. Offices are core and do not need this feature.

use suvorov_core::{Date, Error, Feature, FeatureSet, Person, Stance, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Diplomacy)).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world
        .add_person(Person::new(vec!["Alder".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
    world
}

#[test]
fn a_ruler_is_core_and_a_stance_needs_diplomacy() {
    let mut world = world();
    world.appoint(0, "ruler", 0).unwrap();
    assert_eq!(world.office_holder(0, "ruler").unwrap(), Some(0));
    world.set_stance(0, 1, Stance::War).unwrap();
    assert_eq!(world.stance(1, 0).unwrap(), Stance::War);
    world.set_opinion(0, 1, -40).unwrap();
    assert_eq!(world.opinion(0, 1).unwrap(), -40);
    assert_eq!(world.opinion(1, 0).unwrap(), 0);
}

#[test]
fn diplomacy_off_refuses_a_stance() {
    let mut world = World::new(1001, 1, 1).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_polity("Southfold").unwrap();
    assert!(matches!(world.set_stance(0, 1, Stance::War), Err(Error::FeatureDisabled(_))));
}

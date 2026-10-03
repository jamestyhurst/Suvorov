//! Opt-in features: a dynastic profile can marry and inherit; a state-war profile cannot.

use suvorov_core::{
    profile_crusader_kings_like, profile_hearts_of_iron_like, Date, Effect, Error, Feature,
    FeatureSet, Person, World,
};

fn person(name: &str) -> Person {
    Person::new(
        vec![name.into()],
        vec![0],
        Date::new(1000, 1, 1),
        0,
        0,
    )
}

fn world_with(features: FeatureSet) -> World {
    let mut world = World::with_features(1001, 1, 1, features).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world.add_person(person("Alder")).unwrap();
    world.add_person(person("Birch")).unwrap();
    world
}

#[test]
fn hearts_of_iron_profile_refuses_dynastic_calls() {
    let profile = profile_hearts_of_iron_like();
    let mut world = world_with(profile.features);
    assert!(world.features().contains(Feature::Scripting));
    assert!(!world.features().contains(Feature::Marriage));
    assert!(matches!(
        world.contract_marriage(0, 1),
        Err(Error::FeatureDisabled(_))
    ));
    assert!(matches!(
        world.create_title("Northfold", Some(0)),
        Err(Error::FeatureDisabled(_))
    ));
}

#[test]
fn crusader_kings_profile_marries_and_passes_a_title() {
    let profile = profile_crusader_kings_like();
    let mut world = world_with(profile.features.clone());
    world.contract_marriage(0, 1).unwrap();
    assert_eq!(world.spouse(0).unwrap(), Some(1));
    assert_eq!(world.spouse(1).unwrap(), Some(0));

    let title = world.create_title("Northfold", Some(0)).unwrap();
    assert_eq!(world.title_name(title).unwrap(), "Northfold");
    world.designate_heir(title, 1).unwrap();
    world.kill_person(0).unwrap();
    assert_eq!(world.title_holder(title).unwrap(), Some(1));
    assert!(world.spouse(1).unwrap().is_none());
}

#[test]
fn titles_without_inheritance_stay_with_the_dead() {
    let features = FeatureSet::none().enable(Feature::Titles);
    let mut world = world_with(features);
    let title = world.create_title("Northfold", Some(0)).unwrap();
    assert_eq!(world.title_name(title).unwrap(), "Northfold");
    assert!(matches!(
        world.designate_heir(title, 1),
        Err(Error::FeatureDisabled(_))
    ));
    world.kill_person(0).unwrap();
    assert_eq!(world.title_holder(title).unwrap(), Some(0));
}

#[test]
fn hearts_of_iron_profile_runs_a_bound_rune_script() {
    let mut world = world_with(FeatureSet::hearts_of_iron_like());
    world
        .bind_script(
            "on_pulse",
            "pub fn on_fire(year, month, day) { if year == 1001 { \"pulse\" } else { \"wrong\" } }",
        )
        .unwrap();
    world
        .schedule_event_with_effects(
            Date::new(1001, 1, 2),
            "pulse",
            vec![Effect::RunScript("on_pulse".into())],
        )
        .unwrap();
    world.advance_one_day();
    assert_eq!(
        world.drain_fired_scripts(),
        vec!["on_pulse=pulse".to_string()]
    );
}

#[test]
fn scripting_off_rejects_a_binding() {
    let mut world = world_with(FeatureSet::none());
    assert!(matches!(
        world.bind_script("on_pulse", "nope"),
        Err(Error::FeatureDisabled(_))
    ));
}

#[test]
fn bare_world_starts_with_no_optional_features() {
    let world = World::new(1001, 1, 1).unwrap();
    assert!(!world.features().contains(Feature::Marriage));
    assert!(!world.features().contains(Feature::Scripting));
}

#[test]
fn a_script_can_marry_only_when_that_feature_is_on() {
    let mut world = world_with(FeatureSet::none().enable(Feature::Scripting).enable(Feature::Marriage));
    world
        .bind_script(
            "on_pulse",
            "pub fn on_fire(year, month, day) { world::contract_marriage(0, 1) }",
        )
        .unwrap();
    world
        .schedule_event_with_effects(
            Date::new(1001, 1, 2),
            "pulse",
            vec![Effect::RunScript("on_pulse".into())],
        )
        .unwrap();
    world.advance_one_day();
    assert_eq!(world.drain_fired_scripts(), vec!["on_pulse=ok".to_string()]);
    assert_eq!(world.spouse(0).unwrap(), Some(1));
}

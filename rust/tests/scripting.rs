//! Isolated suite for Scripting. Rune runs. Marriage stays off.

use suvorov_core::{Date, Effect, Error, Feature, FeatureSet, Person, World};

fn world() -> World {
    let mut world = World::with_features(1001, 1, 1, FeatureSet::none().enable(Feature::Scripting)).unwrap();
    world.add_polity("Northfold").unwrap();
    world.add_location("Aldergate").unwrap();
    world
        .add_person(Person::new(vec!["Alder".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
    world
}

#[test]
fn rune_on_fire_runs_when_the_event_date_arrives() {
    let mut world = world();
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
fn a_script_that_is_not_rune_is_rejected_at_bind() {
    let mut world = world();
    assert!(matches!(
        world.bind_script("broken", "this is not rune"),
        Err(Error::Script(_))
    ));
}

#[test]
fn scripting_mode_still_refuses_marriage() {
    let mut world = world();
    assert!(matches!(
        world.contract_marriage(0, 0),
        Err(Error::FeatureDisabled(_))
    ));
}

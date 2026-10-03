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

#[test]
fn a_script_reads_the_world_date_through_the_binding() {
    let mut world = world();
    world
        .bind_script(
            "on_pulse",
            "pub fn on_fire(year, month, day) { world::date_text() }",
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
        vec!["on_pulse=1001-1-2".to_string()]
    );
}

#[test]
fn a_script_cannot_marry_when_marriage_is_off() {
    let mut world = world();
    world
        .add_person(Person::new(vec!["Birch".into()], vec![0], Date::new(1000, 1, 1), 0, 0))
        .unwrap();
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
    assert_eq!(
        world.drain_fired_scripts(),
        vec!["on_pulse=refused".to_string()]
    );
    assert!(matches!(world.spouse(0), Err(Error::FeatureDisabled(_))));
}

#[test]
fn a_script_reads_a_name_and_cannot_grant_a_title() {
    let mut world = world();
    world
        .bind_script(
            "on_pulse",
            "pub fn on_fire(year, month, day) { if world::grant_title(0, \"Northfold\") < 0 { \"refused\" } else { \"ok\" } }",
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
    assert_eq!(world.drain_fired_scripts(), vec!["on_pulse=refused".to_string()]);
}

#[test]
fn a_script_can_move_and_rename_allegiance_without_dynastic_features() {
    let mut world = world();
    world.add_polity("Southfold").unwrap();
    world.add_location("Birchford").unwrap();
    world
        .bind_script(
            "on_pulse",
            "pub fn on_fire(year, month, day) { world::move_person(0, 1); world::set_allegiance(0, 1); world::person_name(0) }",
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
    assert_eq!(world.drain_fired_scripts(), vec!["on_pulse=Alder".to_string()]);
    assert_eq!(world.person(0).unwrap().current_location_id, 1);
    assert_eq!(world.person(0).unwrap().allegiances, vec![1]);
}

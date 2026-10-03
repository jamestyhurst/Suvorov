use suvorov_core::{world_from_records, CharacterRecord, Date, TitleRecord};

fn aurora_records() -> (Vec<TitleRecord>, Vec<CharacterRecord>) {
    let titles = vec![
        TitleRecord {
            id: "d_aurora".into(),
            holder: Some("calen".into()),
        },
        TitleRecord {
            id: "c_vale".into(),
            holder: Some("calen".into()),
        },
    ];
    let characters = vec![
        CharacterRecord {
            id: "calen".into(),
            name: "Calen".into(),
            birth: "0980-03-01".into(),
            death: None,
        },
        CharacterRecord {
            id: "mira".into(),
            name: "Mira".into(),
            birth: "0988-06-15".into(),
            death: None,
        },
        CharacterRecord {
            id: "old_ryn".into(),
            name: "Ryn".into(),
            birth: "0940-01-01".into(),
            death: Some("0999-12-31".into()),
        },
    ];
    (titles, characters)
}

#[test]
fn holders_become_persons_with_title_allegiance() {
    let (titles, characters) = aurora_records();
    let loaded = world_from_records("1000-01-01", &titles, &characters).unwrap();

    assert!(loaded.person_ids.contains_key("calen"));
    assert!(loaded.person_ids.contains_key("mira"));

    let calen_id = loaded.person_ids["calen"];
    let mira_id = loaded.person_ids["mira"];
    let calen = loaded.world.person(calen_id).unwrap();
    assert_eq!(calen.names, vec!["Calen"]);
    assert_eq!(calen.allegiances.len(), 2);
    assert_eq!(loaded.world.person_age(calen_id).unwrap(), 19);
    assert_eq!(loaded.world.person_age(mira_id).unwrap(), 11);

    let mira = loaded.world.person(mira_id).unwrap();
    assert_eq!(mira.allegiances, vec![loaded.polity_ids["unlanded"]]);
}

#[test]
fn already_dead_characters_stay_in_the_world() {
    let (titles, characters) = aurora_records();
    let loaded = world_from_records("1000-01-01", &titles, &characters).unwrap();

    assert!(loaded.person_ids.contains_key("old_ryn"));
    let ryn_id = loaded.person_ids["old_ryn"];
    assert!(!loaded.world.is_alive(ryn_id).unwrap());
    assert_eq!(
        loaded.world.person_death_date(ryn_id).unwrap(),
        Some(Date::new(999, 12, 31))
    );
    assert_eq!(loaded.world.person_count(), 3);
    assert_eq!(loaded.world.living_person_count(), 2);
}

#[test]
fn tick_after_load_rolls_a_non_leap_february() {
    let (titles, characters) = aurora_records();
    let mut loaded = world_from_records("1000-02-28", &titles, &characters).unwrap();
    loaded.world.advance_one_day();
    // 1000 is not a Gregorian leap year (divisible by 100, not 400).
    assert_eq!(loaded.world.date(), Date::new(1000, 3, 1));
}

use suvorov_core::{Error, World};

/// Four locations in a line: a - b - c - d, two polities.
fn line_world() -> (World, [u32; 4], [u32; 2]) {
    let mut world = World::new(2024, 1, 1).unwrap();
    let aurora = world.add_polity("Aurora").unwrap();
    let helia = world.add_polity("Helia").unwrap();
    let ids = [
        world.add_location("Amber Vale").unwrap(),
        world.add_location("Glass Ridge").unwrap(),
        world.add_location("Mist Ford").unwrap(),
        world.add_location("Iron Reach").unwrap(),
    ];
    for pair in ids.windows(2) {
        world.connect_locations(pair[0], pair[1]).unwrap();
    }
    (world, ids, [aurora, helia])
}

#[test]
fn adjacency_is_symmetric_and_deduplicated() {
    let (mut world, ids, _) = line_world();
    assert_eq!(world.neighbors(ids[1]).unwrap(), &[ids[0], ids[2]]);
    world.connect_locations(ids[2], ids[1]).unwrap();
    assert_eq!(world.neighbors(ids[1]).unwrap(), &[ids[0], ids[2]]);
}

#[test]
fn connect_rejects_self_and_unknown_locations() {
    let (mut world, ids, _) = line_world();
    assert!(matches!(
        world.connect_locations(ids[0], ids[0]),
        Err(Error::InvalidArgument(_))
    ));
    assert!(matches!(
        world.connect_locations(ids[0], 99),
        Err(Error::OutOfRange(_))
    ));
}

#[test]
fn ownership_starts_unowned_and_validates_ids() {
    let (mut world, ids, polities) = line_world();
    assert_eq!(world.location_owner(ids[0]).unwrap(), None);
    world.set_location_owner(ids[0], Some(polities[0])).unwrap();
    assert_eq!(world.location_owner(ids[0]).unwrap(), Some(polities[0]));
    assert!(matches!(
        world.set_location_owner(ids[0], Some(99)),
        Err(Error::OutOfRange(_))
    ));
    assert!(matches!(
        world.set_location_owner(99, None),
        Err(Error::OutOfRange(_))
    ));
}

#[test]
fn borders_are_derived_from_ownership_and_move_when_it_changes() {
    let (mut world, ids, [aurora, helia]) = line_world();
    // Unowned land has no borders.
    assert!(world.borders().is_empty());

    world.set_location_owner(ids[0], Some(aurora)).unwrap();
    world.set_location_owner(ids[1], Some(aurora)).unwrap();
    world.set_location_owner(ids[2], Some(helia)).unwrap();
    // Unowned ids[3] touches Helia but is not a border.
    assert_eq!(world.borders(), vec![(ids[1], ids[2])]);

    // Transfer ids[1]: the border shifts without any border data being edited.
    world.set_location_owner(ids[1], Some(helia)).unwrap();
    assert_eq!(world.borders(), vec![(ids[0], ids[1])]);
}

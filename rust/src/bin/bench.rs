use std::env;
use std::time::Instant;
use suvorov_core::{Date, Person, World};

fn scenario(persons: usize, locations: usize, polities: usize, days: usize) -> (i64, Date) {
    let mut world = World::new(1000, 1, 1).expect("valid start");
    let mut polity_ids = Vec::with_capacity(polities);
    for i in 0..polities {
        polity_ids.push(world.add_polity(format!("Polity{i}")).unwrap());
    }
    let mut location_ids = Vec::with_capacity(locations);
    for i in 0..locations {
        location_ids.push(world.add_location(format!("Place{i}")).unwrap());
    }
    for i in 0..persons {
        let birth_year = 950 + (i % 50) as i32;
        let person = Person::new(
            vec![format!("Person{i}")],
            vec![polity_ids[i % polities]],
            Date::new(birth_year, 3, 1),
            location_ids[i % locations],
            location_ids[i % locations],
        );
        world.add_person(person).unwrap();
    }

    let mut checksum: i64 = 0;
    for day in 0..days {
        world.advance_one_day();
        for pid in 0..persons as u32 {
            checksum += world.person_age(pid).unwrap() as i64;
            let person = world.person(pid).unwrap();
            checksum += person.allegiances[0] as i64;
        }
        if persons > 0 {
            let mover = (day % persons) as u32;
            let dest = location_ids[(day + 1) % locations];
            world.set_person_location(mover, dest).unwrap();
        }
    }
    (checksum, world.date())
}

fn clock_only(days: usize) -> Date {
    let mut world = World::new(1000, 1, 1).unwrap();
    for _ in 0..days {
        world.advance_one_day();
    }
    world.date()
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "full".into());
    match mode.as_str() {
        "clock" => {
            let days: usize = args.next().unwrap_or_else(|| "1000000".into()).parse().unwrap();
            let start = Instant::now();
            let date = clock_only(days);
            let ms = start.elapsed().as_secs_f64() * 1000.0;
            println!("lang=rust scenario=clock days={days} ms={ms:.3} date={}-{}-{}", date.year, date.month, date.day);
        }
        _ => {
            let persons: usize = args.next().unwrap_or_else(|| "20000".into()).parse().unwrap();
            let locations: usize = args.next().unwrap_or_else(|| "200".into()).parse().unwrap();
            let polities: usize = args.next().unwrap_or_else(|| "50".into()).parse().unwrap();
            let days: usize = args.next().unwrap_or_else(|| "365".into()).parse().unwrap();
            let start = Instant::now();
            let (checksum, date) = scenario(persons, locations, polities, days);
            let ms = start.elapsed().as_secs_f64() * 1000.0;
            println!(
                "lang=rust scenario=scan persons={persons} locations={locations} polities={polities} days={days} ms={ms:.3} checksum={checksum} date={}-{}-{}",
                date.year, date.month, date.day
            );
        }
    }
}

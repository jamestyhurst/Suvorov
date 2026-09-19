#include <cassert>
#include <stdexcept>
#include <string>

#include "suvorov/core/world.hpp"

int main() {
    suvorov::World world{2024, 1, 1};

    // Placeholder names are fictional; AGENTS.md forbids real places or peoples in tests.
    const auto polity_id = world.add_polity("Aurora");
    assert(world.polity_name(polity_id) == "Aurora");

    const auto second_id = world.add_polity("Helia");
    assert(second_id != polity_id);
    assert(world.polity_name(second_id) == "Helia");

    const auto person_id = world.add_person("Calen", polity_id);
    assert(world.person_name(person_id) == "Calen");
    assert(world.person_polity(person_id) == polity_id);

    const auto same_polity_person = world.add_person("Mira", polity_id);
    assert(same_polity_person != person_id);
    assert(world.person_polity(same_polity_person) == polity_id);

    const auto other_polity_person = world.add_person("Ryn", second_id);
    assert(world.person_polity(other_polity_person) == second_id);

    world.advance_one_day();

    const auto date = world.date();
    assert(date.year == 2024);
    assert(date.month == 1);
    assert(date.day == 2);
    assert(world.person_name(person_id) == "Calen");
    assert(world.person_polity(person_id) == polity_id);

    suvorov::World end_of_year{2024, 12, 31};
    end_of_year.advance_one_day();
    const auto next_year = end_of_year.date();
    assert(next_year.year == 2025);
    assert(next_year.month == 1);
    assert(next_year.day == 1);

    suvorov::World leap_day{2024, 2, 28};
    leap_day.advance_one_day();
    const auto feb_29 = leap_day.date();
    assert(feb_29.year == 2024);
    assert(feb_29.month == 2);
    assert(feb_29.day == 29);
    leap_day.advance_one_day();
    const auto leap_march = leap_day.date();
    assert(leap_march.year == 2024);
    assert(leap_march.month == 3);
    assert(leap_march.day == 1);

    suvorov::World non_leap{2023, 2, 28};
    non_leap.advance_one_day();
    const auto non_leap_march = non_leap.date();
    assert(non_leap_march.year == 2023);
    assert(non_leap_march.month == 3);
    assert(non_leap_march.day == 1);

    bool rejected_invalid_date = false;
    try {
        suvorov::World invalid_date{2024, 2, 30};
    } catch (const std::invalid_argument&) {
        rejected_invalid_date = true;
    }
    assert(rejected_invalid_date);

    bool rejected_empty_name = false;
    try {
        world.add_polity("");
    } catch (const std::invalid_argument&) {
        rejected_empty_name = true;
    }
    assert(rejected_empty_name);

    bool rejected_unknown_id = false;
    try {
        (void)world.polity_name(99);
    } catch (const std::out_of_range&) {
        rejected_unknown_id = true;
    }
    assert(rejected_unknown_id);

    bool rejected_empty_person_name = false;
    try {
        world.add_person("", polity_id);
    } catch (const std::invalid_argument&) {
        rejected_empty_person_name = true;
    }
    assert(rejected_empty_person_name);

    bool rejected_unknown_person_polity = false;
    try {
        world.add_person("Calen", 99);
    } catch (const std::out_of_range&) {
        rejected_unknown_person_polity = true;
    }
    assert(rejected_unknown_person_polity);

    bool rejected_unknown_person = false;
    try {
        (void)world.person_name(99);
    } catch (const std::out_of_range&) {
        rejected_unknown_person = true;
    }
    assert(rejected_unknown_person);

    bool rejected_unknown_person_polity_lookup = false;
    try {
        (void)world.person_polity(99);
    } catch (const std::out_of_range&) {
        rejected_unknown_person_polity_lookup = true;
    }
    assert(rejected_unknown_person_polity_lookup);
}

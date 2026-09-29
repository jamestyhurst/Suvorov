#include <cassert>
#include <stdexcept>
#include <string>
#include <vector>

#include "world.hpp"

int main() {
    suvorov::World world{2024, 1, 1};

    const auto polity_id = world.add_polity("Aurora");
    assert(world.polity_name(polity_id) == "Aurora");

    const auto second_id = world.add_polity("Helia");
    assert(second_id != polity_id);
    assert(world.polity_name(second_id) == "Helia");

    const auto vale = world.add_location("Amber Vale");
    assert(world.location_name(vale) == "Amber Vale");
    const auto ridge = world.add_location("Glass Ridge");
    assert(ridge != vale);

    suvorov::Person calen;
    calen.names = {"Calen", "of Vale"};
    calen.allegiances = {polity_id, second_id};
    calen.birth_date = {2000, 3, 1};
    calen.birth_location_id = vale;
    calen.current_location_id = vale;

    const auto person_id = world.add_person(calen);
    const auto& stored_calen = world.person(person_id);
    assert(stored_calen.names.size() == 2);
    assert(stored_calen.names.front() == "Calen");
    assert(stored_calen.names.back() == "of Vale");
    assert(stored_calen.allegiances.size() == 2);
    assert(world.person_age(person_id) == 23);

    suvorov::Person mira;
    mira.names = {"Mira"};
    mira.allegiances = {polity_id};
    mira.birth_date = {2000, 3, 1};
    mira.birth_location_id = vale;
    mira.current_location_id = ridge;
    const auto mira_id = world.add_person(mira);
    assert(mira_id != person_id);

    world.set_person_location(person_id, ridge);
    assert(world.person(person_id).current_location_id == ridge);

    world.advance_one_day();
    const auto date = world.date();
    assert(date.year == 2024 && date.month == 1 && date.day == 2);

    suvorov::World birthday_world{2024, 2, 29};
    const auto birthday_polity = birthday_world.add_polity("Aurora");
    const auto birthday_place = birthday_world.add_location("Amber Vale");
    suvorov::Person turning;
    turning.names = {"Calen"};
    turning.allegiances = {birthday_polity};
    turning.birth_date = {2000, 3, 1};
    turning.birth_location_id = birthday_place;
    turning.current_location_id = birthday_place;
    const auto turning_id = birthday_world.add_person(turning);
    assert(birthday_world.person_age(turning_id) == 23);
    birthday_world.advance_one_day();
    assert((birthday_world.date() == suvorov::Date{2024, 3, 1}));
    assert(birthday_world.person_age(turning_id) == 24);

    suvorov::World born_today{2024, 6, 15};
    const auto infant_polity = born_today.add_polity("Helia");
    const auto infant_place = born_today.add_location("Glass Ridge");
    suvorov::Person infant;
    infant.names = {"Ryn"};
    infant.allegiances = {infant_polity};
    infant.birth_date = {2024, 6, 15};
    infant.birth_location_id = infant_place;
    infant.current_location_id = infant_place;
    assert(born_today.person_age(born_today.add_person(infant)) == 0);

    assert(suvorov::biological_age({2000, 2, 29}, {2023, 2, 28}) == 22);
    assert(suvorov::biological_age({2000, 2, 29}, {2023, 3, 1}) == 23);

    suvorov::World end_of_year{2024, 12, 31};
    end_of_year.advance_one_day();
    assert((end_of_year.date() == suvorov::Date{2025, 1, 1}));

    suvorov::World leap_day{2024, 2, 28};
    leap_day.advance_one_day();
    assert((leap_day.date() == suvorov::Date{2024, 2, 29}));
    leap_day.advance_one_day();
    assert((leap_day.date() == suvorov::Date{2024, 3, 1}));

    suvorov::World non_leap{2023, 2, 28};
    non_leap.advance_one_day();
    assert((non_leap.date() == suvorov::Date{2023, 3, 1}));

    bool rejected_invalid_date = false;
    try {
        suvorov::World invalid_date{2024, 2, 30};
    } catch (const std::invalid_argument&) {
        rejected_invalid_date = true;
    }
    assert(rejected_invalid_date);

    bool rejected_empty_polity_name = false;
    try {
        world.add_polity("");
    } catch (const std::invalid_argument&) {
        rejected_empty_polity_name = true;
    }
    assert(rejected_empty_polity_name);

    bool rejected_unknown_polity = false;
    try {
        (void)world.polity_name(99);
    } catch (const std::out_of_range&) {
        rejected_unknown_polity = true;
    }
    assert(rejected_unknown_polity);

    bool rejected_no_names = false;
    try {
        suvorov::Person no_names = mira;
        no_names.names.clear();
        world.add_person(no_names);
    } catch (const std::invalid_argument&) {
        rejected_no_names = true;
    }
    assert(rejected_no_names);

    bool rejected_unknown_person = false;
    try {
        (void)world.person(99);
    } catch (const std::out_of_range&) {
        rejected_unknown_person = true;
    }
    assert(rejected_unknown_person);

    bool rejected_move_unknown_place = false;
    try {
        world.set_person_location(person_id, 99);
    } catch (const std::out_of_range&) {
        rejected_move_unknown_place = true;
    }
    assert(rejected_move_unknown_place);

    return 0;
}

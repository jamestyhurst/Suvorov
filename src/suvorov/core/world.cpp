#include "suvorov/core/world.hpp"

#include <stdexcept>
#include <utility>

namespace suvorov {
namespace {

bool is_leap_year(int year) {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

int days_in_month(int year, int month) {
    switch (month) {
    case 2:
        return is_leap_year(year) ? 29 : 28;
    case 4:
    case 6:
    case 9:
    case 11:
        return 30;
    default:
        return 31;
    }
}

}  // namespace

void Date::advance_one_day() {
    ++day;
    if (day <= days_in_month(year, month)) {
        return;
    }

    day = 1;
    ++month;
    if (month <= 12) {
        return;
    }

    month = 1;
    ++year;
}

World::World(int year, int month, int day)
    : current_date_{year, month, day} {
    if (month < 1 || month > 12 || day < 1 ||
        day > days_in_month(year, month)) {
        throw std::invalid_argument("World date is invalid");
    }
}

std::uint32_t World::add_polity(std::string name) {
    if (name.empty()) {
        throw std::invalid_argument("Polity name cannot be empty");
    }

    polities_.push_back(std::move(name));
    return static_cast<std::uint32_t>(polities_.size() - 1);
}

const std::string& World::polity_name(std::uint32_t polity_id) const {
    if (polity_id >= polities_.size()) {
        throw std::out_of_range("Polity id does not exist");
    }

    return polities_[polity_id];
}

std::uint32_t World::add_person(std::string name, std::uint32_t polity_id) {
    if (name.empty()) {
        throw std::invalid_argument("Person name cannot be empty");
    }
    if (polity_id >= polities_.size()) {
        throw std::out_of_range("Polity id does not exist");
    }

    persons_.push_back(Person{std::move(name), polity_id});
    return static_cast<std::uint32_t>(persons_.size() - 1);
}

const std::string& World::person_name(std::uint32_t person_id) const {
    if (person_id >= persons_.size()) {
        throw std::out_of_range("Person id does not exist");
    }

    return persons_[person_id].name;
}

std::uint32_t World::person_polity(std::uint32_t person_id) const {
    if (person_id >= persons_.size()) {
        throw std::out_of_range("Person id does not exist");
    }

    return persons_[person_id].polity_id;
}

void World::advance_one_day() {
    current_date_.advance_one_day();
}

Date World::date() const {
    return current_date_;
}

}  // namespace suvorov

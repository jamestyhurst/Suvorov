#include "suvorov/core/world.hpp"

#include <stdexcept>
#include <utility>

namespace suvorov {

World::World(int year, int month, int day) : current_date_{year, month, day} {
    if (!current_date_.valid()) {
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

std::uint32_t World::add_location(std::string name) {
    if (name.empty()) {
        throw std::invalid_argument("Location name cannot be empty");
    }

    locations_.push_back(std::move(name));
    return static_cast<std::uint32_t>(locations_.size() - 1);
}

const std::string& World::location_name(std::uint32_t location_id) const {
    if (location_id >= locations_.size()) {
        throw std::out_of_range("Location id does not exist");
    }

    return locations_[location_id];
}

std::uint32_t World::add_person(Person person) {
    if (person.names.empty()) {
        throw std::invalid_argument("Person must have at least one name");
    }
    for (const auto& name : person.names) {
        if (name.empty()) {
            throw std::invalid_argument("Person name cannot be empty");
        }
    }

    if (person.allegiances.empty()) {
        throw std::invalid_argument("Person must have at least one allegiance");
    }
    for (const auto allegiance : person.allegiances) {
        if (allegiance >= polities_.size()) {
            throw std::out_of_range("Polity id does not exist");
        }
    }

    if (!person.birth_date.valid()) {
        throw std::invalid_argument("Person birth date is invalid");
    }
    if (current_date_ < person.birth_date) {
        throw std::invalid_argument("Person birth date is after the world date");
    }

    if (person.birth_location_id >= locations_.size() ||
        person.current_location_id >= locations_.size()) {
        throw std::out_of_range("Location id does not exist");
    }

    persons_.push_back(std::move(person));
    return static_cast<std::uint32_t>(persons_.size() - 1);
}

const Person& World::person(std::uint32_t person_id) const {
    if (person_id >= persons_.size()) {
        throw std::out_of_range("Person id does not exist");
    }

    return persons_[person_id];
}

int World::person_age(std::uint32_t person_id) const {
    return biological_age(person(person_id).birth_date, current_date_);
}

void World::set_person_location(std::uint32_t person_id,
                               std::uint32_t location_id) {
    if (person_id >= persons_.size()) {
        throw std::out_of_range("Person id does not exist");
    }
    if (location_id >= locations_.size()) {
        throw std::out_of_range("Location id does not exist");
    }

    persons_[person_id].current_location_id = location_id;
}

void World::advance_one_day() {
    current_date_.advance_one_day();
}

Date World::date() const {
    return current_date_;
}

}  // namespace suvorov

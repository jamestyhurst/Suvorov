#pragma once

#include "suvorov/core/person.hpp"

#include <cstdint>
#include <string>
#include <vector>

namespace suvorov {

class World {
public:
    World(int year, int month, int day);

    std::uint32_t add_polity(std::string name);
    const std::string& polity_name(std::uint32_t polity_id) const;

    std::uint32_t add_location(std::string name);
    const std::string& location_name(std::uint32_t location_id) const;

    std::uint32_t add_person(Person person);
    const Person& person(std::uint32_t person_id) const;
    int person_age(std::uint32_t person_id) const;
    void set_person_location(std::uint32_t person_id, std::uint32_t location_id);

    void advance_one_day();
    Date date() const;

private:
    Date current_date_;
    std::vector<std::string> polities_;
    std::vector<std::string> locations_;
    std::vector<Person> persons_;
};

}  // namespace suvorov

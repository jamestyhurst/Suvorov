#pragma once

#include "suvorov/core/date.hpp"

#include <cstdint>
#include <string>
#include <vector>

namespace suvorov {

struct Person {
    std::vector<std::string> names;
    std::vector<std::uint32_t> allegiances;
    Date birth_date;
    std::uint32_t birth_location_id;
    std::uint32_t current_location_id;
};

}  // namespace suvorov

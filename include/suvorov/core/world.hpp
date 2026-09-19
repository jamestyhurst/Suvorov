#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace suvorov {

struct Date {
    int year;
    int month;
    int day;

    void advance_one_day();
};

class World {
public:
    World(int year, int month, int day);

    std::uint32_t add_polity(std::string name);
    const std::string& polity_name(std::uint32_t polity_id) const;
    void advance_one_day();
    Date date() const;

private:
    Date current_date_;
    std::vector<std::string> polities_;
};

}  // namespace suvorov

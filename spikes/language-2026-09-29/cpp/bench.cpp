#include "world.hpp"

#include <chrono>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

namespace {

suvorov::Date clock_only(std::size_t days) {
    suvorov::World world{1000, 1, 1};
    for (std::size_t i = 0; i < days; ++i) {
        world.advance_one_day();
    }
    return world.date();
}

std::pair<long long, suvorov::Date> scenario(std::size_t persons, std::size_t locations,
                                             std::size_t polities, std::size_t days) {
    suvorov::World world{1000, 1, 1};
    std::vector<std::uint32_t> polity_ids;
    polity_ids.reserve(polities);
    for (std::size_t i = 0; i < polities; ++i) {
        polity_ids.push_back(world.add_polity("Polity" + std::to_string(i)));
    }
    std::vector<std::uint32_t> location_ids;
    location_ids.reserve(locations);
    for (std::size_t i = 0; i < locations; ++i) {
        location_ids.push_back(world.add_location("Place" + std::to_string(i)));
    }
    for (std::size_t i = 0; i < persons; ++i) {
        suvorov::Person person;
        person.names = {"Person" + std::to_string(i)};
        person.allegiances = {polity_ids[i % polities]};
        person.birth_date = {static_cast<int>(950 + (i % 50)), 3, 1};
        person.birth_location_id = location_ids[i % locations];
        person.current_location_id = location_ids[i % locations];
        world.add_person(std::move(person));
    }

    long long checksum = 0;
    for (std::size_t day = 0; day < days; ++day) {
        world.advance_one_day();
        for (std::uint32_t pid = 0; pid < persons; ++pid) {
            checksum += world.person_age(pid);
            checksum += world.person(pid).allegiances.front();
        }
        if (persons > 0) {
            const auto mover = static_cast<std::uint32_t>(day % persons);
            const auto dest = location_ids[(day + 1) % locations];
            world.set_person_location(mover, dest);
        }
    }
    return {checksum, world.date()};
}

}  // namespace

int main(int argc, char** argv) {
    const std::string mode = argc > 1 ? argv[1] : "scan";
    if (mode == "clock") {
        const std::size_t days = argc > 2 ? std::stoull(argv[2]) : 1000000ULL;
        const auto start = std::chrono::steady_clock::now();
        const auto date = clock_only(days);
        const double ms = std::chrono::duration<double, std::milli>(
                              std::chrono::steady_clock::now() - start)
                              .count();
        std::cout << "lang=cpp scenario=clock days=" << days << " ms=" << ms
                  << " date=" << date.year << "-" << date.month << "-" << date.day
                  << "\n";
        return 0;
    }

    const std::size_t persons = argc > 2 ? std::stoull(argv[2]) : 20000ULL;
    const std::size_t locations = argc > 3 ? std::stoull(argv[3]) : 200ULL;
    const std::size_t polities = argc > 4 ? std::stoull(argv[4]) : 50ULL;
    const std::size_t days = argc > 5 ? std::stoull(argv[5]) : 365ULL;
    const auto start = std::chrono::steady_clock::now();
    const auto [checksum, date] = scenario(persons, locations, polities, days);
    const double ms = std::chrono::duration<double, std::milli>(
                          std::chrono::steady_clock::now() - start)
                          .count();
    std::cout << "lang=cpp scenario=scan persons=" << persons
              << " locations=" << locations << " polities=" << polities
              << " days=" << days << " ms=" << ms << " checksum=" << checksum
              << " date=" << date.year << "-" << date.month << "-" << date.day
              << "\n";
    return 0;
}

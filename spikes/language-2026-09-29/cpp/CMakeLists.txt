cmake_minimum_required(VERSION 3.20)
project(suvorov_language_spike LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(suvorov_spike_core date.cpp world.cpp)
target_include_directories(suvorov_spike_core PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(suvorov_spike_tests world_tests.cpp)
target_link_libraries(suvorov_spike_tests PRIVATE suvorov_spike_core)

add_executable(suvorov_spike_bench bench.cpp)
target_link_libraries(suvorov_spike_bench PRIVATE suvorov_spike_core)

enable_testing()
add_test(NAME suvorov_spike_tests COMMAND suvorov_spike_tests)

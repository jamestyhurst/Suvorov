cmake_minimum_required(VERSION 3.20)

project(Suvorov LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(suvorov_core
    src/suvorov/core/world.cpp
)

target_include_directories(suvorov_core
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# BUILD_TESTING is CMake's conventional on/off switch. ON registers the test
# executable with CTest; OFF builds only the suvorov_core library.
if (BUILD_TESTING)
    enable_testing()

    add_executable(suvorov_core_tests
        tests/world_tests.cpp
    )
    target_link_libraries(suvorov_core_tests PRIVATE suvorov_core)

    add_test(NAME suvorov_core_tests COMMAND suvorov_core_tests)
endif()

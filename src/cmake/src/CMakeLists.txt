cmake_minimum_required(VERSION 3.22.0)

project(hello_world)

add_executable(${PROJECT_NAME} hello_world.cpp)

find_package(nlohmann_json 3.2.0 REQUIRED)
target_link_libraries(hello_world PRIVATE nlohmann_json::nlohmann_json)

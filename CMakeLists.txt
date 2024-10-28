cmake_minimum_required(VERSION 3.10)
project(WTINet)

set(CMAKE_CXX_STANDARD 17)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

include_directories(${Qt5Widgets_INCLUDE_DIRS})
include_directories(${Boost_INCLUDE_DIRS})

add_executable(WTINet main.cpp WTINetGui.cpp WTINetGui.hpp WTIDLMS.cpp WTIDLMS.hpp HTTPOWTIN.cpp HTTPOWTIN.hpp MLMLS.cpp MLMLS.hpp)
target_link_libraries(WTINet Qt5::Widgets)

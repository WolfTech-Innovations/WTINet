cmake_minimum_required(VERSION 3.10)
project(WTINet)

set(CMAKE_CXX_STANDARD 17)

# Include src directory for header files
include_directories(src)

# Add all source files from src directory
file(GLOB SOURCES "src/*.cpp")

# Create executable from sources
add_executable(WTINet ${SOURCES})

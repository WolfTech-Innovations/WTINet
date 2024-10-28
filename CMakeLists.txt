cmake_minimum_required(VERSION 3.10)
project(WTINet)

set(CMAKE_CXX_STANDARD 17)

add_executable(WTINet main.cpp WTIDLMS.cpp HTTPOWTIN.cpp MLMLS.cpp)

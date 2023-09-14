cmake_minimum_required(VERSION 3.25)

project(libpci-rs-backend C CXX)

if(APPLE)
    add_library(libpci-rs-backend SHARED darwin/mod.cpp)
else()
    message(FATAL_ERROR "Attempt to compile on unknown target!")
endif()

target_include_directories(libpci-rs-backend PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
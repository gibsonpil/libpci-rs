cmake_minimum_required(VERSION 3.25)

project(libpci-rs-c-backend C CXX)

include(CTest)

set(FLAGS "-O2")
set(CMAKE_CXX_FLAGS ${FLAGS})
set(CMAKE_C_FLAGS ${FLAGS})
set(CTEST_OUTPUT_ON_FAILURE 1)

set(SOURCE_FILES common.c)

if(APPLE)
    set(SOURCE_FILES ${SOURCE_FILES} darwin/mod.cpp)
elseif(BSD STREQUAL "OpenBSD")
    set(SOURCE_FILES ${SOURCE_FILES} bsd/mod.c)
else()
    set(SOURCE_FILES ${SOURCE_FILES} stub.c)
    message(WARNING "Attempt to compile on unknown target, including stub API.")
endif()

add_library(libpci-rs-c-backend STATIC ${SOURCE_FILES})

add_executable(test_stack ${CMAKE_CURRENT_SOURCE_DIR}/tests/test_stack.c)
target_link_libraries(test_stack PUBLIC libpci-rs-c-backend)
target_include_directories(test_stack PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/tests/)
add_test(NAME test_stack COMMAND test_stack)

target_include_directories(libpci-rs-c-backend PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS libpci-rs-c-backend DESTINATION .)

enable_testing()

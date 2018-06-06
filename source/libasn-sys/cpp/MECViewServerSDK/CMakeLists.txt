cmake_minimum_required(VERSION 3.5)

project (MECViewServerSDK)

set(CMAKE_CXX_STANDARD 11)

set(VERSION_MAJOR "1")
set(VERSION_MINOR "0")
set(VERSION_PATCH "0")

include(../common/version.cmake)
include(../common/gcc_5.4.0_includes.cmake)
include(../common/global.cmake)
include(../common/doxygen.cmake)

#-------------------------------------------------------------------------------------------------
include(../common/install_prefix.cmake)

install (FILES "${CMAKE_CURRENT_BINARY_DIR}/version.h" DESTINATION "include/mecview-sdk")

add_subdirectory(${CMAKE_SOURCE_DIR}/proto)
add_subdirectory(${CMAKE_SOURCE_DIR}/extension)
add_subdirectory(${CMAKE_SOURCE_DIR}/algorithm)
add_subdirectory(${CMAKE_SOURCE_DIR}/road_clearance)

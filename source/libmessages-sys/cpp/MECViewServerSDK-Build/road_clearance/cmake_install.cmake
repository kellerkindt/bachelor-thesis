# Install script for directory: /home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/road_clearance

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK-Build")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

if(NOT CMAKE_INSTALL_COMPONENT OR "${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified")
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mecview-sdk/road_clearance" TYPE FILE FILES
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/road_clearance/road_clearance_module_factory.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/road_clearance/road_clearance_listener.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/road_clearance/road_clearance_input.h"
    )
endif()


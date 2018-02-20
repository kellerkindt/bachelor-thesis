cmake_minimum_required(VERSION 3.5)

project (MECViewServerSDK)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_INSTALL_PREFIX ${CMAKE_BINARY_DIR})

set(VERSION_MAJOR "0")
set(VERSION_MINOR "1")
set(VERSION_PATCH "0")

# Add the svn branch and revision number to a header file for versioning -----------------------------------------

# Set the current version
set(VERSION_SHORT "${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")

message("Version: " ${VERSION_SHORT})

if(EXISTS ${CMAKE_SOURCE_DIR}/.svn)
        include(FindSubversion)
        Subversion_WC_INFO(${CMAKE_CURRENT_SOURCE_DIR} MY)

endif(EXISTS ${CMAKE_SOURCE_DIR}/.svn)

message("Revision: " ${MY_WC_REVISION})

configure_file(
         ${CMAKE_SOURCE_DIR}/version.h.in
        ${CMAKE_BINARY_DIR}/version.h
)

# Configure doxygen for source code documentation -----------------------------------------------------------

find_package(Doxygen)
option(BUILD_DOCUMENTATION "Create and install the HTML based API documentation (requires Doxygen)" ${DOXYGEN_FOUND})

if(BUILD_DOCUMENTATION)
        if(NOT DOXYGEN_FOUND)
                message(FATAL_ERROR "Doxygen is needed to build the documentation.")
        endif()

        set(doxyfile_in ${CMAKE_CURRENT_SOURCE_DIR}/Doxyfile.in)
        set(doxyfile ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile)

        configure_file(${doxyfile_in} ${doxyfile} @ONLY)

        add_custom_target(doc
                COMMAND ${DOXYGEN_EXECUTABLE} ${doxyfile}
                WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
                COMMENT "Generating API documentation with Doxygen"
                VERBATIM)
endif()
# ------------------------------------------------------------------------------------------------------------

install (FILES "${CMAKE_CURRENT_BINARY_DIR}/version.h" DESTINATION "include/mecview-sdk")

add_subdirectory(${CMAKE_SOURCE_DIR}/proto)
add_subdirectory(${CMAKE_SOURCE_DIR}/extension)
add_subdirectory(${CMAKE_SOURCE_DIR}/algorithm)
add_subdirectory(${CMAKE_SOURCE_DIR}/road_clearance)

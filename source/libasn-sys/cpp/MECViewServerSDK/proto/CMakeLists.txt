file(GLOB ASN1_FILES "${CMAKE_CURRENT_SOURCE_DIR}/*.asn1")

#message("Searched: ${CMAKE_CURRENT_SOURCE_DIR} for files: ${ASN1_FILES}")

execute_process(COMMAND asn1c -fcompound-names -Werror -gen-PER ${ASN1_FILES} WORKING_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}")

file(REMOVE "${CMAKE_CURRENT_BINARY_DIR}/converter-example.c")

file(GLOB ASN1_SRC "${CMAKE_CURRENT_BINARY_DIR}/*.c")
file(GLOB ASN1_HDR "${CMAKE_CURRENT_BINARY_DIR}/*.h")

include_directories("${CMAKE_CURRENT_BINARY_DIR}")

add_library(messages STATIC ${ASN1_SRC})

string(TOLOWER ${CMAKE_BUILD_TYPE} BUILD_TYPE_DIR)

install (TARGETS messages DESTINATION lib/${BUILD_TYPE_DIR}/${CMAKE_SYSTEM_PROCESSOR}/)

install (FILES ${ASN1_HDR} DESTINATION "include/mecview-sdk/proto")
install (FILES ${ASN1_FILES} DESTINATION "include/mecview-sdk/proto/asn1")


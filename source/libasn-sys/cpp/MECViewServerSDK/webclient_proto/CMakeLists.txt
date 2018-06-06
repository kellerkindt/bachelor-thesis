# Google protobuf --------------------------------------------------------------------------------------------

find_package(Protobuf REQUIRED)

if(NOT PROTOBUF_FOUND)
        message(FATAL_ERROR "Protobuf library could not be found.")
endif()

include_directories(${PROTOBUF_INCLUDE_DIRS})

message("generating protobuf source files...")
file(GLOB Protofiles "${CMAKE_CURRENT_SOURCE_DIR}/*.proto")
PROTOBUF_GENERATE_CPP(PROTO_SRC PROTO_HDR ${Protofiles})
message("generating protobuf source files... finished. (${PROTO_SRCS})")

add_library(proto_messages STATIC ${PROTO_SRC})
target_link_libraries(proto_messages /usr/local/lib/libprotobuf.a)

#-----------------------------------------------------------------------------------------

install (TARGETS proto_messages DESTINATION lib/${BUILD_TYPE_DIR}/${CMAKE_SYSTEM_PROCESSOR}/)

install (FILES ${PROTO_HDR} DESTINATION "include/mecview-sdk/webclient_proto/proto")

#----------------------------------------------------------------------------------------

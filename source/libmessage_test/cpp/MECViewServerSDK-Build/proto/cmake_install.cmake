# Install script for directory: /home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
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
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/libmessages.a")
endif()

if(NOT CMAKE_INSTALL_COMPONENT OR "${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified")
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mecview-sdk/proto" TYPE FILE FILES
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Header.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ReferencePoint.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/PositionOffset.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/MovingVector.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ObjectSize.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ObjectType.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Covariance.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/PathPoint.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Path.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/PredictedPath.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SectorIds.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Error.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Errors.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/EnvironmentEnvelope.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/EnvironmentObjectDetection.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/EnvironmentObjectDetections.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/EnvironmentFrame.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Area.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Sector.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Sectors.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/InitMessage.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/RoadClearanceEnvelope.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Parameter.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/Parameters.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/RoadSection.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/RoadSections.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/RoadClearanceFrame.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ClientType.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/CoveredArea.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SubscriptionStatus.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ClientRegistration.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/UpdateSubscription.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ConnectionStatus.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/UpdateStatus.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorType.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorEnvelope.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorObjectDetection.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorObjectDetections.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorFrame.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/SensorIdleFrame.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/UTF8String.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/BOOLEAN.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/INTEGER.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/NativeEnumerated.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/NativeInteger.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_SEQUENCE_OF.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_SET_OF.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/constr_SEQUENCE.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/constr_SEQUENCE_OF.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/constr_SET_OF.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_application.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_system.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_codecs.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_internal.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/OCTET_STRING.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/BIT_STRING.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/asn_codecs_prim.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ber_tlv_length.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ber_tlv_tag.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/ber_decoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/der_encoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/constr_TYPE.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/constraints.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/xer_support.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/xer_decoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/xer_encoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/per_support.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/per_decoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/per_encoder.h"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK-Build/proto/per_opentype.h"
    )
endif()

if(NOT CMAKE_INSTALL_COMPONENT OR "${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified")
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mecview-sdk/proto/asn1" TYPE FILE FILES
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/general.asn1"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/environment.asn1"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/road_clearance.asn1"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/protocol.asn1"
    "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/sensor.asn1"
    )
endif()


# Install script for directory: /home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
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

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/release/x86_64" TYPE STATIC_LIBRARY FILES "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/libmessages.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mecview-sdk/proto" TYPE FILE FILES
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Area.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/BIT_STRING.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/BOOLEAN.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ClientRegistration.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ClientType.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ConnectionStatus.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Covariance.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/CoveredArea.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/EnvironmentEnvelope.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/EnvironmentFrame.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/EnvironmentObjectDetection.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/EnvironmentObjectDetections.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Error.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Errors.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Header.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/INTEGER.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/InitMessage.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/MovingVector.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/NativeEnumerated.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/NativeInteger.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/OCTET_STRING.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ObjectSize.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ObjectType.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Parameter.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Parameters.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Path.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/PathPoint.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/PositionOffset.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/PredictedPath.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ReferencePoint.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/RoadClearanceEnvelope.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/RoadClearanceFrame.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/RoadSection.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/RoadSections.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Sector.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SectorIds.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/Sectors.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorEnvelope.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorFrame.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorIdleFrame.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorObjectDetection.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorObjectDetections.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SensorType.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/SubscriptionStatus.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/UTF8String.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/UpdateStatus.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/UpdateSubscription.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_SEQUENCE_OF.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_SET_OF.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_application.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_codecs.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_codecs_prim.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_internal.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/asn_system.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ber_decoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ber_tlv_length.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/ber_tlv_tag.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/constr_SEQUENCE.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/constr_SEQUENCE_OF.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/constr_SET_OF.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/constr_TYPE.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/constraints.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/der_encoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/per_decoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/per_encoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/per_opentype.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/per_support.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/xer_decoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/xer_encoder.h"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK-Build/proto/xer_support.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mecview-sdk/proto/asn1" TYPE FILE FILES
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/environment.asn1"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/general.asn1"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/protocol.asn1"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/road_clearance.asn1"
    "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/sensor.asn1"
    )
endif()


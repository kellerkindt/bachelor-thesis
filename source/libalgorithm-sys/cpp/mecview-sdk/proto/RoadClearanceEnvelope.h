/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewRoadClearanceModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libsdk-sys/cpp/MECViewServerSDK/proto/road_clearance.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_RoadClearanceEnvelope_H_
#define	_RoadClearanceEnvelope_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeInteger.h>
#include "ReferencePoint.h"
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* RoadClearanceEnvelope */
typedef struct RoadClearanceEnvelope {
	long	 version;
	long	 server_id;
	ReferencePoint_t	 reference_point;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} RoadClearanceEnvelope_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_RoadClearanceEnvelope;

#ifdef __cplusplus
}
#endif

#endif	/* _RoadClearanceEnvelope_H_ */
#include <asn_internal.h>

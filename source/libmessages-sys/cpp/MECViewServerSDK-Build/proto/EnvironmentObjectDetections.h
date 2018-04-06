/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_EnvironmentObjectDetections_H_
#define	_EnvironmentObjectDetections_H_


#include <asn_application.h>

/* Including external dependencies */
#include <asn_SEQUENCE_OF.h>
#include <constr_SEQUENCE_OF.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct EnvironmentObjectDetection;

/* EnvironmentObjectDetections */
typedef struct EnvironmentObjectDetections {
	A_SEQUENCE_OF(struct EnvironmentObjectDetection) list;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} EnvironmentObjectDetections_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_EnvironmentObjectDetections;

#ifdef __cplusplus
}
#endif

/* Referred external types */
#include "EnvironmentObjectDetection.h"

#endif	/* _EnvironmentObjectDetections_H_ */
#include <asn_internal.h>

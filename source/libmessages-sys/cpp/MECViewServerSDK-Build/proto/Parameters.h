/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewRoadClearanceModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/proto/road_clearance.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_Parameters_H_
#define	_Parameters_H_


#include <asn_application.h>

/* Including external dependencies */
#include <asn_SEQUENCE_OF.h>
#include <constr_SEQUENCE_OF.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct Parameter;

/* Parameters */
typedef struct Parameters {
	A_SEQUENCE_OF(struct Parameter) list;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} Parameters_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_Parameters;

#ifdef __cplusplus
}
#endif

/* Referred external types */
#include "Parameter.h"

#endif	/* _Parameters_H_ */
#include <asn_internal.h>

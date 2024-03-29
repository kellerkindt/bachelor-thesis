/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_Path_H_
#define	_Path_H_


#include <asn_application.h>

/* Including external dependencies */
#include <asn_SEQUENCE_OF.h>
#include <constr_SEQUENCE_OF.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct PathPoint;

/* Path */
typedef struct Path {
	A_SEQUENCE_OF(struct PathPoint) list;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} Path_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_Path;

#ifdef __cplusplus
}
#endif

/* Referred external types */
#include "PathPoint.h"

#endif	/* _Path_H_ */
#include <asn_internal.h>

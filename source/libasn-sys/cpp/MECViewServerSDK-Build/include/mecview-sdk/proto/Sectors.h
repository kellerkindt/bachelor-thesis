/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_Sectors_H_
#define	_Sectors_H_


#include <asn_application.h>

/* Including external dependencies */
#include <asn_SEQUENCE_OF.h>
#include <constr_SEQUENCE_OF.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct Sector;

/* Sectors */
typedef struct Sectors {
	A_SEQUENCE_OF(struct Sector) list;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} Sectors_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_Sectors;

#ifdef __cplusplus
}
#endif

/* Referred external types */
#include "Sector.h"

#endif	/* _Sectors_H_ */
#include <asn_internal.h>

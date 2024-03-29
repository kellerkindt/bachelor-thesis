/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewGeneralModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/general.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_ReferencePoint_H_
#define	_ReferencePoint_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeInteger.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ReferencePoint */
typedef struct ReferencePoint {
	long	 latitude;
	long	 longitude;
	long	 altitude;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} ReferencePoint_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_ReferencePoint;

#ifdef __cplusplus
}
#endif

#endif	/* _ReferencePoint_H_ */
#include <asn_internal.h>

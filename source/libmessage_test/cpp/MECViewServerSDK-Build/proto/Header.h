/*
 * Generated by asn1c-0.9.24 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewGeneralModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/general.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types`
 */

#ifndef	_Header_H_
#define	_Header_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeInteger.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Header */
typedef struct Header {
	long	 timestamp;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} Header_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_Header;

#ifdef __cplusplus
}
#endif

#endif	/* _Header_H_ */
#include <asn_internal.h>

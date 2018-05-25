/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libsdk-sys/cpp/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_InitMessage_H_
#define	_InitMessage_H_


#include <asn_application.h>

/* Including external dependencies */
#include "Header.h"
#include "EnvironmentEnvelope.h"
#include "Sectors.h"
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* InitMessage */
typedef struct InitMessage {
	Header_t	 header;
	EnvironmentEnvelope_t	 envelope;
	Sectors_t	 sectors;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} InitMessage_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_InitMessage;

#ifdef __cplusplus
}
#endif

#endif	/* _InitMessage_H_ */
#include <asn_internal.h>

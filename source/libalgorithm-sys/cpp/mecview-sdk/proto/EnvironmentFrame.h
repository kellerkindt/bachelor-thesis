/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libsdk-sys/cpp/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_EnvironmentFrame_H_
#define	_EnvironmentFrame_H_


#include <asn_application.h>

/* Including external dependencies */
#include "Header.h"
#include "EnvironmentEnvelope.h"
#include "EnvironmentObjectDetections.h"
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* EnvironmentFrame */
typedef struct EnvironmentFrame {
	Header_t	 header;
	EnvironmentEnvelope_t	 envelope;
	EnvironmentObjectDetections_t	 object_detections;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} EnvironmentFrame_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_EnvironmentFrame;

#ifdef __cplusplus
}
#endif

#endif	/* _EnvironmentFrame_H_ */
#include <asn_internal.h>
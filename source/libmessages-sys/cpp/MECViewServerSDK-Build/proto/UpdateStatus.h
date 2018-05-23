/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_UpdateStatus_H_
#define	_UpdateStatus_H_


#include <asn_application.h>

/* Including external dependencies */
#include "ConnectionStatus.h"
#include <UTF8String.h>
#include <NativeInteger.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* UpdateStatus */
typedef struct UpdateStatus {
	ConnectionStatus_t	 sensor_status;
	UTF8String_t	 ip_address;
	long	 sensor_id;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} UpdateStatus_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_UpdateStatus;

#ifdef __cplusplus
}
#endif

#endif	/* _UpdateStatus_H_ */
#include <asn_internal.h>

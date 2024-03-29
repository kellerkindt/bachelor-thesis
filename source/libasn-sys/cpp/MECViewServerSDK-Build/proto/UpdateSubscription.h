/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_UpdateSubscription_H_
#define	_UpdateSubscription_H_


#include <asn_application.h>

/* Including external dependencies */
#include "SubscriptionStatus.h"
#include <NativeInteger.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* UpdateSubscription */
typedef struct UpdateSubscription {
	SubscriptionStatus_t	 subscription_status;
	long	*message_period	/* OPTIONAL */;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} UpdateSubscription_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_UpdateSubscription;

#ifdef __cplusplus
}
#endif

#endif	/* _UpdateSubscription_H_ */
#include <asn_internal.h>

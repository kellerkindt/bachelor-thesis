/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_Covariance_H_
#define	_Covariance_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeInteger.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Covariance */
typedef struct Covariance {
	long	*north_pos_east_pos	/* OPTIONAL */;
	long	*north_pos_north_vel	/* OPTIONAL */;
	long	*north_pos_east_vel	/* OPTIONAL */;
	long	*east_pos_north_vel	/* OPTIONAL */;
	long	*east_pos_east_vel	/* OPTIONAL */;
	long	*north_vel_east_vel	/* OPTIONAL */;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} Covariance_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_Covariance;

#ifdef __cplusplus
}
#endif

#endif	/* _Covariance_H_ */
#include <asn_internal.h>

/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_ClientType_H_
#define	_ClientType_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeEnumerated.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum ClientType {
	ClientType_sensor	= 0,
	ClientType_vehicle	= 1
} e_ClientType;

/* ClientType */
typedef long	 ClientType_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_ClientType;
asn_struct_free_f ClientType_free;
asn_struct_print_f ClientType_print;
asn_constr_check_f ClientType_constraint;
ber_type_decoder_f ClientType_decode_ber;
der_type_encoder_f ClientType_encode_der;
xer_type_decoder_f ClientType_decode_xer;
xer_type_encoder_f ClientType_encode_xer;
per_type_decoder_f ClientType_decode_uper;
per_type_encoder_f ClientType_encode_uper;

#ifdef __cplusplus
}
#endif

#endif	/* _ClientType_H_ */
#include <asn_internal.h>

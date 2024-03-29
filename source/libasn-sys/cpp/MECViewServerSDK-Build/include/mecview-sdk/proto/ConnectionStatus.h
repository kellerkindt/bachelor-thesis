/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#ifndef	_ConnectionStatus_H_
#define	_ConnectionStatus_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeEnumerated.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum ConnectionStatus {
	ConnectionStatus_connected	= 0,
	ConnectionStatus_disconnected	= 1,
	ConnectionStatus_faulty	= 2
} e_ConnectionStatus;

/* ConnectionStatus */
typedef long	 ConnectionStatus_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_ConnectionStatus;
asn_struct_free_f ConnectionStatus_free;
asn_struct_print_f ConnectionStatus_print;
asn_constr_check_f ConnectionStatus_constraint;
ber_type_decoder_f ConnectionStatus_decode_ber;
der_type_encoder_f ConnectionStatus_encode_der;
xer_type_decoder_f ConnectionStatus_decode_xer;
xer_type_encoder_f ConnectionStatus_encode_xer;
per_type_decoder_f ConnectionStatus_decode_uper;
per_type_encoder_f ConnectionStatus_encode_uper;

#ifdef __cplusplus
}
#endif

#endif	/* _ConnectionStatus_H_ */
#include <asn_internal.h>

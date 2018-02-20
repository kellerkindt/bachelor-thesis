/*
 * Generated by asn1c-0.9.24 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types`
 */

#include "ClientRegistration.h"

static int
memb_minimum_message_period_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 10000)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static asn_per_constraints_t asn_PER_memb_minimum_message_period_constr_4 = {
	{ APC_CONSTRAINED,	 14,  14,  0,  10000 }	/* (0..10000) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_TYPE_member_t asn_MBR_ClientRegistration_1[] = {
	{ ATF_NOFLAGS, 0, offsetof(struct ClientRegistration, type),
		(ASN_TAG_CLASS_CONTEXT | (0 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_ClientType,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"type"
		},
	{ ATF_POINTER, 2, offsetof(struct ClientRegistration, covered_area),
		(ASN_TAG_CLASS_CONTEXT | (1 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_CoveredArea,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"covered-area"
		},
	{ ATF_POINTER, 1, offsetof(struct ClientRegistration, minimum_message_period),
		(ASN_TAG_CLASS_CONTEXT | (2 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_minimum_message_period_constraint_1,
		&asn_PER_memb_minimum_message_period_constr_4,
		0,
		"minimum-message-period"
		},
};
static int asn_MAP_ClientRegistration_oms_1[] = { 1, 2 };
static ber_tlv_tag_t asn_DEF_ClientRegistration_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static asn_TYPE_tag2member_t asn_MAP_ClientRegistration_tag2el_1[] = {
    { (ASN_TAG_CLASS_CONTEXT | (0 << 2)), 0, 0, 0 }, /* type at 21 */
    { (ASN_TAG_CLASS_CONTEXT | (1 << 2)), 1, 0, 0 }, /* covered-area at 22 */
    { (ASN_TAG_CLASS_CONTEXT | (2 << 2)), 2, 0, 0 } /* minimum-message-period at 23 */
};
static asn_SEQUENCE_specifics_t asn_SPC_ClientRegistration_specs_1 = {
	sizeof(struct ClientRegistration),
	offsetof(struct ClientRegistration, _asn_ctx),
	asn_MAP_ClientRegistration_tag2el_1,
	3,	/* Count of tags in the map */
	asn_MAP_ClientRegistration_oms_1,	/* Optional members */
	2, 0,	/* Root/Additions */
	-1,	/* Start extensions */
	-1	/* Stop extensions */
};
asn_TYPE_descriptor_t asn_DEF_ClientRegistration = {
	"ClientRegistration",
	"ClientRegistration",
	SEQUENCE_free,
	SEQUENCE_print,
	SEQUENCE_constraint,
	SEQUENCE_decode_ber,
	SEQUENCE_encode_der,
	SEQUENCE_decode_xer,
	SEQUENCE_encode_xer,
	SEQUENCE_decode_uper,
	SEQUENCE_encode_uper,
	0,	/* Use generic outmost tag fetcher */
	asn_DEF_ClientRegistration_tags_1,
	sizeof(asn_DEF_ClientRegistration_tags_1)
		/sizeof(asn_DEF_ClientRegistration_tags_1[0]), /* 1 */
	asn_DEF_ClientRegistration_tags_1,	/* Same as above */
	sizeof(asn_DEF_ClientRegistration_tags_1)
		/sizeof(asn_DEF_ClientRegistration_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_ClientRegistration_1,
	3,	/* Elements count */
	&asn_SPC_ClientRegistration_specs_1	/* Additional specs */
};

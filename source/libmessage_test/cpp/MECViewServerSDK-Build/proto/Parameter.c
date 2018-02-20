/*
 * Generated by asn1c-0.9.24 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewRoadClearanceModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/ffi_test/libmessage_test/cpp/MECViewServerSDK/proto/road_clearance.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types`
 */

#include "Parameter.h"

static int
memb_id_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 255)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static int
memb_value_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= -10 && value <= 10)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		_ASN_CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static asn_per_constraints_t asn_PER_memb_id_constr_2 = {
	{ APC_CONSTRAINED,	 8,  8,  0,  255 }	/* (0..255) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_value_constr_4 = {
	{ APC_CONSTRAINED,	 5,  5, -10,  10 }	/* (-10..10) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_TYPE_member_t asn_MBR_Parameter_1[] = {
	{ ATF_NOFLAGS, 0, offsetof(struct Parameter, id),
		(ASN_TAG_CLASS_CONTEXT | (0 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_id_constraint_1,
		&asn_PER_memb_id_constr_2,
		0,
		"id"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct Parameter, name),
		(ASN_TAG_CLASS_CONTEXT | (1 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_UTF8String,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"name"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct Parameter, value),
		(ASN_TAG_CLASS_CONTEXT | (2 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_value_constraint_1,
		&asn_PER_memb_value_constr_4,
		0,
		"value"
		},
	{ ATF_POINTER, 1, offsetof(struct Parameter, position),
		(ASN_TAG_CLASS_CONTEXT | (3 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_ReferencePoint,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"position"
		},
};
static int asn_MAP_Parameter_oms_1[] = { 3 };
static ber_tlv_tag_t asn_DEF_Parameter_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static asn_TYPE_tag2member_t asn_MAP_Parameter_tag2el_1[] = {
    { (ASN_TAG_CLASS_CONTEXT | (0 << 2)), 0, 0, 0 }, /* id at 15 */
    { (ASN_TAG_CLASS_CONTEXT | (1 << 2)), 1, 0, 0 }, /* name at 16 */
    { (ASN_TAG_CLASS_CONTEXT | (2 << 2)), 2, 0, 0 }, /* value at 17 */
    { (ASN_TAG_CLASS_CONTEXT | (3 << 2)), 3, 0, 0 } /* position at 18 */
};
static asn_SEQUENCE_specifics_t asn_SPC_Parameter_specs_1 = {
	sizeof(struct Parameter),
	offsetof(struct Parameter, _asn_ctx),
	asn_MAP_Parameter_tag2el_1,
	4,	/* Count of tags in the map */
	asn_MAP_Parameter_oms_1,	/* Optional members */
	1, 0,	/* Root/Additions */
	-1,	/* Start extensions */
	-1	/* Stop extensions */
};
asn_TYPE_descriptor_t asn_DEF_Parameter = {
	"Parameter",
	"Parameter",
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
	asn_DEF_Parameter_tags_1,
	sizeof(asn_DEF_Parameter_tags_1)
		/sizeof(asn_DEF_Parameter_tags_1[0]), /* 1 */
	asn_DEF_Parameter_tags_1,	/* Same as above */
	sizeof(asn_DEF_Parameter_tags_1)
		/sizeof(asn_DEF_Parameter_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_Parameter_1,
	4,	/* Elements count */
	&asn_SPC_Parameter_specs_1	/* Additional specs */
};


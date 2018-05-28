/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#include "EnvironmentEnvelope.h"

static int
memb_version_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static int
memb_server_id_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static asn_per_constraints_t asn_PER_memb_version_constr_2 GCC_NOTUSED = {
	{ APC_SEMI_CONSTRAINED,	-1, -1,  0,  0 }	/* (0..MAX) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_server_id_constr_3 GCC_NOTUSED = {
	{ APC_SEMI_CONSTRAINED,	-1, -1,  0,  0 }	/* (0..MAX) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_TYPE_member_t asn_MBR_EnvironmentEnvelope_1[] = {
	{ ATF_NOFLAGS, 0, offsetof(struct EnvironmentEnvelope, version),
		(ASN_TAG_CLASS_CONTEXT | (0 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_version_constraint_1,
		&asn_PER_memb_version_constr_2,
		0,
		"version"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct EnvironmentEnvelope, server_id),
		(ASN_TAG_CLASS_CONTEXT | (1 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_server_id_constraint_1,
		&asn_PER_memb_server_id_constr_3,
		0,
		"server-id"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct EnvironmentEnvelope, reference_point),
		(ASN_TAG_CLASS_CONTEXT | (2 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_ReferencePoint,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"reference-point"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct EnvironmentEnvelope, error_codes),
		(ASN_TAG_CLASS_CONTEXT | (3 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_Errors,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"error-codes"
		},
};
static const ber_tlv_tag_t asn_DEF_EnvironmentEnvelope_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static const asn_TYPE_tag2member_t asn_MAP_EnvironmentEnvelope_tag2el_1[] = {
    { (ASN_TAG_CLASS_CONTEXT | (0 << 2)), 0, 0, 0 }, /* version */
    { (ASN_TAG_CLASS_CONTEXT | (1 << 2)), 1, 0, 0 }, /* server-id */
    { (ASN_TAG_CLASS_CONTEXT | (2 << 2)), 2, 0, 0 }, /* reference-point */
    { (ASN_TAG_CLASS_CONTEXT | (3 << 2)), 3, 0, 0 } /* error-codes */
};
static asn_SEQUENCE_specifics_t asn_SPC_EnvironmentEnvelope_specs_1 = {
	sizeof(struct EnvironmentEnvelope),
	offsetof(struct EnvironmentEnvelope, _asn_ctx),
	asn_MAP_EnvironmentEnvelope_tag2el_1,
	4,	/* Count of tags in the map */
	0, 0, 0,	/* Optional elements (not needed) */
	-1,	/* Start extensions */
	-1	/* Stop extensions */
};
asn_TYPE_descriptor_t asn_DEF_EnvironmentEnvelope = {
	"EnvironmentEnvelope",
	"EnvironmentEnvelope",
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
	asn_DEF_EnvironmentEnvelope_tags_1,
	sizeof(asn_DEF_EnvironmentEnvelope_tags_1)
		/sizeof(asn_DEF_EnvironmentEnvelope_tags_1[0]), /* 1 */
	asn_DEF_EnvironmentEnvelope_tags_1,	/* Same as above */
	sizeof(asn_DEF_EnvironmentEnvelope_tags_1)
		/sizeof(asn_DEF_EnvironmentEnvelope_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_EnvironmentEnvelope_1,
	4,	/* Elements count */
	&asn_SPC_EnvironmentEnvelope_specs_1	/* Additional specs */
};


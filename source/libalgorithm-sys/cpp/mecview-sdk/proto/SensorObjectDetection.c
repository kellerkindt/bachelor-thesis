/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewSensorModule"
 * 	found in "/home/mi7wa6/Dokumente/mecview_2018-05-18/MECViewServerSDK/proto/sensor.asn1"
 * 	`asn1c -fcompound-names -Werror -gen-PER`
 */

#include "SensorObjectDetection.h"

static int
memb_local_id_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
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
memb_probability_of_existence_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 1000000)) {
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
memb_type_probability_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 1000000)) {
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
memb_orientation_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 3600)) {
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
memb_std_dev_orientation_constraint_1(asn_TYPE_descriptor_t *td, const void *sptr,
			asn_app_constraint_failed_f *ctfailcb, void *app_key) {
	long value;
	
	if(!sptr) {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: value not given (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
	
	value = *(const long *)sptr;
	
	if((value >= 0 && value <= 1800)) {
		/* Constraint check succeeded */
		return 0;
	} else {
		ASN__CTFAIL(app_key, td, sptr,
			"%s: constraint failed (%s:%d)",
			td->name, __FILE__, __LINE__);
		return -1;
	}
}

static asn_per_constraints_t asn_PER_memb_local_id_constr_2 GCC_NOTUSED = {
	{ APC_SEMI_CONSTRAINED,	-1, -1,  0,  0 }	/* (0..MAX) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_probability_of_existence_constr_3 GCC_NOTUSED = {
	{ APC_CONSTRAINED,	 20, -1,  0,  1000000 }	/* (0..1000000) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_type_probability_constr_7 GCC_NOTUSED = {
	{ APC_CONSTRAINED,	 20, -1,  0,  1000000 }	/* (0..1000000) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_orientation_constr_9 GCC_NOTUSED = {
	{ APC_CONSTRAINED,	 12,  12,  0,  3600 }	/* (0..3600) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_per_constraints_t asn_PER_memb_std_dev_orientation_constr_10 GCC_NOTUSED = {
	{ APC_CONSTRAINED,	 11,  11,  0,  1800 }	/* (0..1800) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
static asn_TYPE_member_t asn_MBR_SensorObjectDetection_1[] = {
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, local_id),
		(ASN_TAG_CLASS_CONTEXT | (0 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_local_id_constraint_1,
		&asn_PER_memb_local_id_constr_2,
		0,
		"local-id"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, probability_of_existence),
		(ASN_TAG_CLASS_CONTEXT | (1 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_probability_of_existence_constraint_1,
		&asn_PER_memb_probability_of_existence_constr_3,
		0,
		"probability-of-existence"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, position_offset),
		(ASN_TAG_CLASS_CONTEXT | (2 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_PositionOffset,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"position-offset"
		},
	{ ATF_POINTER, 1, offsetof(struct SensorObjectDetection, moving_vector),
		(ASN_TAG_CLASS_CONTEXT | (3 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_MovingVector,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"moving-vector"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, type),
		(ASN_TAG_CLASS_CONTEXT | (4 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_ObjectType,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"type"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, type_probability),
		(ASN_TAG_CLASS_CONTEXT | (5 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_type_probability_constraint_1,
		&asn_PER_memb_type_probability_constr_7,
		0,
		"type-probability"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct SensorObjectDetection, size),
		(ASN_TAG_CLASS_CONTEXT | (6 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_ObjectSize,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"size"
		},
	{ ATF_POINTER, 3, offsetof(struct SensorObjectDetection, orientation),
		(ASN_TAG_CLASS_CONTEXT | (7 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_orientation_constraint_1,
		&asn_PER_memb_orientation_constr_9,
		0,
		"orientation"
		},
	{ ATF_POINTER, 2, offsetof(struct SensorObjectDetection, std_dev_orientation),
		(ASN_TAG_CLASS_CONTEXT | (8 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_NativeInteger,
		memb_std_dev_orientation_constraint_1,
		&asn_PER_memb_std_dev_orientation_constr_10,
		0,
		"std-dev-orientation"
		},
	{ ATF_POINTER, 1, offsetof(struct SensorObjectDetection, measured),
		(ASN_TAG_CLASS_CONTEXT | (9 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_BOOLEAN,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"measured"
		},
};
static const int asn_MAP_SensorObjectDetection_oms_1[] = { 3, 7, 8, 9 };
static const ber_tlv_tag_t asn_DEF_SensorObjectDetection_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static const asn_TYPE_tag2member_t asn_MAP_SensorObjectDetection_tag2el_1[] = {
    { (ASN_TAG_CLASS_CONTEXT | (0 << 2)), 0, 0, 0 }, /* local-id */
    { (ASN_TAG_CLASS_CONTEXT | (1 << 2)), 1, 0, 0 }, /* probability-of-existence */
    { (ASN_TAG_CLASS_CONTEXT | (2 << 2)), 2, 0, 0 }, /* position-offset */
    { (ASN_TAG_CLASS_CONTEXT | (3 << 2)), 3, 0, 0 }, /* moving-vector */
    { (ASN_TAG_CLASS_CONTEXT | (4 << 2)), 4, 0, 0 }, /* type */
    { (ASN_TAG_CLASS_CONTEXT | (5 << 2)), 5, 0, 0 }, /* type-probability */
    { (ASN_TAG_CLASS_CONTEXT | (6 << 2)), 6, 0, 0 }, /* size */
    { (ASN_TAG_CLASS_CONTEXT | (7 << 2)), 7, 0, 0 }, /* orientation */
    { (ASN_TAG_CLASS_CONTEXT | (8 << 2)), 8, 0, 0 }, /* std-dev-orientation */
    { (ASN_TAG_CLASS_CONTEXT | (9 << 2)), 9, 0, 0 } /* measured */
};
static asn_SEQUENCE_specifics_t asn_SPC_SensorObjectDetection_specs_1 = {
	sizeof(struct SensorObjectDetection),
	offsetof(struct SensorObjectDetection, _asn_ctx),
	asn_MAP_SensorObjectDetection_tag2el_1,
	10,	/* Count of tags in the map */
	asn_MAP_SensorObjectDetection_oms_1,	/* Optional members */
	4, 0,	/* Root/Additions */
	-1,	/* Start extensions */
	-1	/* Stop extensions */
};
asn_TYPE_descriptor_t asn_DEF_SensorObjectDetection = {
	"SensorObjectDetection",
	"SensorObjectDetection",
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
	asn_DEF_SensorObjectDetection_tags_1,
	sizeof(asn_DEF_SensorObjectDetection_tags_1)
		/sizeof(asn_DEF_SensorObjectDetection_tags_1[0]), /* 1 */
	asn_DEF_SensorObjectDetection_tags_1,	/* Same as above */
	sizeof(asn_DEF_SensorObjectDetection_tags_1)
		/sizeof(asn_DEF_SensorObjectDetection_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_SensorObjectDetection_1,
	10,	/* Elements count */
	&asn_SPC_SensorObjectDetection_specs_1	/* Additional specs */
};


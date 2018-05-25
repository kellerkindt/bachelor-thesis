/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewEnvironmentModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libsdk-sys/cpp/MECViewServerSDK/proto/environment.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#include "InitMessage.h"

static asn_TYPE_member_t asn_MBR_InitMessage_1[] = {
	{ ATF_NOFLAGS, 0, offsetof(struct InitMessage, header),
		(ASN_TAG_CLASS_CONTEXT | (0 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_Header,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"header"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct InitMessage, envelope),
		(ASN_TAG_CLASS_CONTEXT | (1 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_EnvironmentEnvelope,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"envelope"
		},
	{ ATF_NOFLAGS, 0, offsetof(struct InitMessage, sectors),
		(ASN_TAG_CLASS_CONTEXT | (2 << 2)),
		-1,	/* IMPLICIT tag at current level */
		&asn_DEF_Sectors,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		"sectors"
		},
};
static const ber_tlv_tag_t asn_DEF_InitMessage_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static const asn_TYPE_tag2member_t asn_MAP_InitMessage_tag2el_1[] = {
    { (ASN_TAG_CLASS_CONTEXT | (0 << 2)), 0, 0, 0 }, /* header */
    { (ASN_TAG_CLASS_CONTEXT | (1 << 2)), 1, 0, 0 }, /* envelope */
    { (ASN_TAG_CLASS_CONTEXT | (2 << 2)), 2, 0, 0 } /* sectors */
};
static asn_SEQUENCE_specifics_t asn_SPC_InitMessage_specs_1 = {
	sizeof(struct InitMessage),
	offsetof(struct InitMessage, _asn_ctx),
	asn_MAP_InitMessage_tag2el_1,
	3,	/* Count of tags in the map */
	0, 0, 0,	/* Optional elements (not needed) */
	-1,	/* Start extensions */
	-1	/* Stop extensions */
};
asn_TYPE_descriptor_t asn_DEF_InitMessage = {
	"InitMessage",
	"InitMessage",
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
	asn_DEF_InitMessage_tags_1,
	sizeof(asn_DEF_InitMessage_tags_1)
		/sizeof(asn_DEF_InitMessage_tags_1[0]), /* 1 */
	asn_DEF_InitMessage_tags_1,	/* Same as above */
	sizeof(asn_DEF_InitMessage_tags_1)
		/sizeof(asn_DEF_InitMessage_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_InitMessage_1,
	3,	/* Elements count */
	&asn_SPC_InitMessage_specs_1	/* Additional specs */
};

/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewProtocolModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libmessages-sys/cpp/MECViewServerSDK/proto/protocol.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#include "CoveredArea.h"

static asn_TYPE_member_t asn_MBR_CoveredArea_1[] = {
	{ ATF_POINTER, 0, 0,
		(ASN_TAG_CLASS_UNIVERSAL | (16 << 2)),
		0,
		&asn_DEF_ReferencePoint,
		0,	/* Defer constraints checking to the member type */
		0,	/* No PER visible constraints */
		0,
		""
		},
};
static const ber_tlv_tag_t asn_DEF_CoveredArea_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (16 << 2))
};
static asn_SET_OF_specifics_t asn_SPC_CoveredArea_specs_1 = {
	sizeof(struct CoveredArea),
	offsetof(struct CoveredArea, _asn_ctx),
	0,	/* XER encoding is XMLDelimitedItemList */
};
asn_TYPE_descriptor_t asn_DEF_CoveredArea = {
	"CoveredArea",
	"CoveredArea",
	SEQUENCE_OF_free,
	SEQUENCE_OF_print,
	SEQUENCE_OF_constraint,
	SEQUENCE_OF_decode_ber,
	SEQUENCE_OF_encode_der,
	SEQUENCE_OF_decode_xer,
	SEQUENCE_OF_encode_xer,
	SEQUENCE_OF_decode_uper,
	SEQUENCE_OF_encode_uper,
	0,	/* Use generic outmost tag fetcher */
	asn_DEF_CoveredArea_tags_1,
	sizeof(asn_DEF_CoveredArea_tags_1)
		/sizeof(asn_DEF_CoveredArea_tags_1[0]), /* 1 */
	asn_DEF_CoveredArea_tags_1,	/* Same as above */
	sizeof(asn_DEF_CoveredArea_tags_1)
		/sizeof(asn_DEF_CoveredArea_tags_1[0]), /* 1 */
	0,	/* No PER visible constraints */
	asn_MBR_CoveredArea_1,
	1,	/* Single element */
	&asn_SPC_CoveredArea_specs_1	/* Additional specs */
};


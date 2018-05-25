/*
 * Generated by asn1c-0.9.28 (http://lionet.info/asn1c)
 * From ASN.1 module "MECViewSensorModule"
 * 	found in "/home/mi7wa6/Dokumente/2018-BA-Watzko/source/libsdk-sys/cpp/MECViewServerSDK/proto/sensor.asn1"
 * 	`asn1c -fcompound-names -fskeletons-copy -fnative-types -gen-PER`
 */

#ifndef	_SensorObjectDetection_H_
#define	_SensorObjectDetection_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeInteger.h>
#include "PositionOffset.h"
#include "ObjectType.h"
#include "ObjectSize.h"
#include <BOOLEAN.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct MovingVector;

/* SensorObjectDetection */
typedef struct SensorObjectDetection {
	long	 local_id;
	long	 probability_of_existence;
	PositionOffset_t	 position_offset;
	struct MovingVector	*moving_vector	/* OPTIONAL */;
	ObjectType_t	 type;
	long	 type_probability;
	ObjectSize_t	 size;
	long	*orientation	/* OPTIONAL */;
	long	*std_dev_orientation	/* OPTIONAL */;
	BOOLEAN_t	*measured	/* OPTIONAL */;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} SensorObjectDetection_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_SensorObjectDetection;

#ifdef __cplusplus
}
#endif

/* Referred external types */
#include "MovingVector.h"

#endif	/* _SensorObjectDetection_H_ */
#include <asn_internal.h>
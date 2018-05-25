/*
 * TestDataProvider.cpp
 *
 *  Created on: Sep 26, 2017
 *      Author: hannes
 */

#include "environment_frame_provider.h"

#include <chrono>
#include <iostream>

namespace mec {
namespace algorithm {

#define MS_PER_DAY 24 * 60 * 60 * 1000

/**
 * Structure to provide the ASN.1 free operation for the EnvironmentFrame datatype.
 */
struct FreeEnvironmentFrame {

	/**
	 * The actual free operation for the EnvironmentFrame datatype.
	 *
	 * @param environment_frame The EnvironmentFrame instance to be freed..
	 */
	void operator()(void* environment_frame) {
		asn_DEF_EnvironmentFrame.free_struct(&asn_DEF_EnvironmentFrame,
				environment_frame, 0);
	}
};

static long GetMillisecondsSinceDay() {
	auto now = std::chrono::system_clock::now();
	std::time_t c_time = std::chrono::system_clock::to_time_t(now);
	std::tm broken_time = *std::localtime(&c_time);

	// reset to midnight...
	broken_time.tm_hour = 0;
	broken_time.tm_min = 0;
	broken_time.tm_sec = 0;

	c_time = std::mktime(&broken_time);

	auto start_of_today = std::chrono::system_clock::from_time_t(c_time);
	auto length_of_today = now - start_of_today;

	// milliseconds since start of day...
	// return (std::chrono::duration_cast<std::chrono::milliseconds>(
	// 			length_of_today)
	// 			.count());
	return (std::chrono::duration_cast<std::chrono::milliseconds>(length_of_today).count() + broken_time.tm_wday * MS_PER_DAY);
}

EnvironmentFrameProvider::EnvironmentFrameProvider() :
		file_path(""), test_data_ready(false), asn1_frame(
				(EnvironmentFrame_t*) calloc(1, sizeof(EnvironmentFrame_t)),
				FreeEnvironmentFrame()), default_asn1_frame(
				(EnvironmentFrame_t*) calloc(1, sizeof(EnvironmentFrame_t)),
				FreeEnvironmentFrame()) {
}

EnvironmentFrameProvider::~EnvironmentFrameProvider() {
}

void EnvironmentFrameProvider::Init(std::string new_file_path) {
	CreateDefaultASN1Frame();

	if (!CreateASN1Frame(new_file_path)) {
		asn1_frame = default_asn1_frame;
	}

	test_data_ready = true;
}

void EnvironmentFrameProvider::SetMessageFilePath(std::string new_file_path) {

	Init(new_file_path);
}

void EnvironmentFrameProvider::CreateDefaultASN1Frame() {
	default_asn1_frame->envelope.version = 1;
	default_asn1_frame->envelope.server_id = 0;

	default_asn1_frame->envelope.reference_point.latitude = 484010822;
	default_asn1_frame->envelope.reference_point.longitude = 99876076;
	default_asn1_frame->envelope.reference_point.altitude = 256000;

	for (int object_count = 0; object_count < 15; object_count++) {

		EnvironmentObjectDetection_t* object_detection =
				(EnvironmentObjectDetection_t*) calloc(1,
						sizeof(EnvironmentObjectDetection_t));

		if (object_detection == NULL) {
			perror("calloc() failed");
			exit(EXIT_FAILURE);
		}

		object_detection->global_id = object_count;
		object_detection->probability_of_existence = 150;

		object_detection->position_offset.position_north = 256;

		long* std_dev_position_north = (long*) calloc(1, sizeof(long));
		*std_dev_position_north = 1;
		object_detection->position_offset.std_dev_position_north =
				std_dev_position_north;

		object_detection->position_offset.position_east = 5000;

		long* std_dev_position_east = (long*) calloc(1, sizeof(long));
		*std_dev_position_east = 1;
		object_detection->position_offset.std_dev_position_east =
				std_dev_position_east;

		MovingVector_t* moving_vector = (MovingVector_t*) calloc(1,
				sizeof(MovingVector_t));

		if (moving_vector == NULL) {
			perror("calloc() failed");
			exit(EXIT_FAILURE);
		}

		moving_vector->velocity_north = 80;
		long* std_dev_velocity_north = (long*) calloc(1, sizeof(long));
		*std_dev_velocity_north = 5;
		moving_vector->std_dev_velocity_north = std_dev_velocity_north;
		moving_vector->velocity_east = 266;
		long* std_dev_velocity_east = (long*) calloc(1, sizeof(long));
		*std_dev_velocity_east = 5;
		moving_vector->std_dev_velocity_east = std_dev_velocity_east;

		object_detection->moving_vector = moving_vector;

		object_detection->type = ObjectType_car;
		object_detection->type_probability = 190;

		// SIZE

		object_detection->size.length = 450;
		long* std_dev_length = (long*) calloc(1, sizeof(long));
		*std_dev_length = 1;
		object_detection->size.std_dev_length = std_dev_length;	// OPTIONAL
		long* width = (long*) calloc(1, sizeof(long));
		*width = 230;
		object_detection->size.width = width;			// OPTIONAL
		long* std_dev_width = (long*) calloc(1, sizeof(long));
		*std_dev_width = 1;
		object_detection->size.std_dev_width = std_dev_width;		// OPTIONAL
		long* height = (long*) calloc(1, sizeof(long));
		*height = 180;
		object_detection->size.height = height;		// OPTIONAL
		long* std_dev_height = (long*) calloc(1, sizeof(long));
		*std_dev_height = 1;
		object_detection->size.std_dev_height = std_dev_height;	// OPTIONAL

		// COVARIANCE

		long* north_pos_east_pos = (long*) calloc(1, sizeof(long));
		*north_pos_east_pos = 1000000;
		object_detection->covariance.north_pos_east_pos = north_pos_east_pos;

		long* north_pos_north_vel = (long*) calloc(1, sizeof(long));
		*north_pos_north_vel = 1000;
		object_detection->covariance.north_pos_north_vel = north_pos_north_vel;

		long* north_pos_east_vel = (long*) calloc(1, sizeof(long));
		*north_pos_east_vel = 1000;
		object_detection->covariance.north_pos_east_vel = north_pos_east_vel;

		long* east_pos_north_vel = (long*) calloc(1, sizeof(long));
		*east_pos_north_vel = 1000;
		object_detection->covariance.east_pos_north_vel = east_pos_north_vel;

		long* east_pos_east_vel = (long*) calloc(1, sizeof(long));
		*east_pos_east_vel = 1000;
		object_detection->covariance.east_pos_east_vel = east_pos_east_vel;

		long* north_vel_east_vel = (long*) calloc(1, sizeof(long));
		*north_vel_east_vel = 100;
		object_detection->covariance.north_vel_east_vel = north_vel_east_vel;

		object_detection->orientation = 25;
		object_detection->std_dev_orientation = 950;
		object_detection->measured = true;

		// PREDICTED PATH

		object_detection->predicted_path.timestamp_dt = 10;

		for (int i = 0; i < 10; ++i) {

			PathPoint_t* pathpoint = (PathPoint_t*) calloc(1,
					sizeof(PathPoint_t));

			if (object_detection == NULL) {
				perror("calloc() failed");
				exit(EXIT_FAILURE);
			}

			pathpoint->position_offset.position_north = 256;

			long* std_dev_position_north = (long*) calloc(1, sizeof(long));
			*std_dev_position_north = 1;
			pathpoint->position_offset.std_dev_position_north =
					std_dev_position_north;

			pathpoint->position_offset.position_east = 5000;

			long* std_dev_position_east = (long*) calloc(1, sizeof(long));
			*std_dev_position_east = 1;
			pathpoint->position_offset.std_dev_position_east =
					std_dev_position_east;

			pathpoint->probability = 195;

			const int result = asn_sequence_add(
					&object_detection->predicted_path.path, pathpoint);

			if (result != 0) {
				perror("asn_set_add() failed");
				exit(EXIT_FAILURE);
			}
		}

		const int result = asn_sequence_add(
				&default_asn1_frame->object_detections, object_detection);
		if (result != 0) {
			perror("asn_set_add() failed");
			exit(EXIT_FAILURE);
		}
	}

	default_asn1_frame->header.timestamp = GetMillisecondsSinceDay();
}

std::shared_ptr<EnvironmentFrame_t> EnvironmentFrameProvider::GetASN1Frame() {

	if (!test_data_ready) {
		Init("");
	}

	asn1_frame->header.timestamp = GetMillisecondsSinceDay();

	return asn1_frame;
}

std::shared_ptr<EnvironmentFrame_t> EnvironmentFrameProvider::GetDefaultASN1Frame() {

	default_asn1_frame->header.timestamp = GetMillisecondsSinceDay();

	return default_asn1_frame;

}

bool EnvironmentFrameProvider::CreateASN1Frame(std::string new_file_path) {
	unsigned int buffer_length = 4194304;
	char* buf = new char[buffer_length]; /* Hope, sufficiently large buffer (for 4mb file) */

	asn_dec_rval_t rval;
	size_t size;
	FILE *f;

	f = fopen(new_file_path.c_str(), "r");
	if (!f) {
		delete[] buf;
		return false;
	}
	size = fread(buf, 1, buffer_length, f);
	if (size == 0 || size == buffer_length) {
		delete[] buf;
		fprintf(stderr, "%s: Too large input\n", new_file_path.c_str());
		return false;
	}

	EnvironmentFrame_t* raw_environment_frame = 0;

	rval = xer_decode(0, &asn_DEF_EnvironmentFrame,
			(void**) (&raw_environment_frame), buf, size);

	delete[] buf;

	if (rval.code != RC_OK) {
		return false;
	}

	this->asn1_frame.reset(raw_environment_frame, FreeEnvironmentFrame());

	xer_fprint(stdout, &asn_DEF_EnvironmentFrame, this->asn1_frame.get());

	return true;
}

} /* namespace util */
} /* namespace mec */

/*
 * road_clearance_input.h
 *
 *  Created on: Oct 27, 2017
 *      Author: hannes
 */

#ifndef ROAD_CLEARANCE_ROAD_CLEARANCE_INPUT_H_
#define ROAD_CLEARANCE_ROAD_CLEARANCE_INPUT_H_

#include <memory>

#include "../proto/EnvironmentFrame.h"

namespace mec {
namespace road_clearance {

/**
 * Road clearance input type enumeration.
 *
 * This is used to determine the type of the inlining data.
 */
enum class RoadClearanceInputType {
	ENVIRONMENT_FRAME, JSON
};

/**
 * Class for abstracting the road clearance input.
 *
 * This is used to provide the road clearance module a queue with different input types. So only
 * one thread is needed to get all the different input possibilities without worrying about synchronization.
 * The class currently provides the input possibility of an environment model and a general json string input.
 */
class RoadClearanceInput {

public:
	/**
	 * Constructor for the environment model input type.
	 *
	 * The ENVIRONMENT_FRAME RoadClearanceInputType is set automatically.
	 *
	 * @param content The environment frame content.
	 *
	 */
	RoadClearanceInput(std::shared_ptr<EnvironmentFrame_t> content) :
			type(RoadClearanceInputType::ENVIRONMENT_FRAME), environment_frame_content(
					content) {
	}

	/**
	 * Constructor for the json string input type.
	 *
	 * The JSON RoadClearanceInputType is set automatically.
	 *
	 * @param content The json string content.
	 *
	 */
	RoadClearanceInput(std::shared_ptr<std::string> content) :
			type(RoadClearanceInputType::JSON), json_content(content) {
	}

	/**
	 * The destructor.
	 */
	virtual ~RoadClearanceInput() {
	}

	/**
	 * Getter for the input type.
	 *
	 * This can be used to determine which operation to use for retrieving the content in the correct format.
	 *
	 * @return The input type (@see RoadClearanceInputType)
	 */
	RoadClearanceInputType GetType() {
		return this->type;
	}

	/**
	 * Getter for environment model content.
	 *
	 * @return The corresponding environment frame (non initialized shared_ptr if the input type is not ENVIRONMENT_FRAME)
	 */
	std::shared_ptr<EnvironmentFrame_t> GetEnvironmentFrameContent() {
		return this->environment_frame_content;
	}

	/**
	 * Getter for json string content.
	 *
	 * @return The corresponding json string (non initialized shared_ptr if the input type is not JSON)
	 */
	std::shared_ptr<std::string> GetJsonContent() {
		return this->json_content;
	}

private:
	RoadClearanceInputType type;
	std::shared_ptr<EnvironmentFrame_t> environment_frame_content;
	std::shared_ptr<std::string> json_content;
};

} /* namespace road_clearance */
} /* namespace mec */

#endif /* ROAD_CLEARANCE_ROAD_CLEARANCE_INPUT_H_ */

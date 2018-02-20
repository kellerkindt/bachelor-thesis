/*
 * algorithm_factory.h
 *
 *  Created on: Nov 10, 2017
 *      Author: Hannes Todenhagen
 *      Email: hannes.todenhagen@it-designers.de
 */

#ifndef ROAD_CLEARANCE_ROAD_CLEARANCE_MODULE_FACTORY_H_
#define ROAD_CLEARANCE_ROAD_CLEARANCE_MODULE_FACTORY_H_

#include <memory>

#include "../proto/EnvironmentFrame.h"

#include "../extension/extension.h"
#include "../extension/queue_listener.h"

#include "road_clearance_input.h"
#include "road_clearance_listener.h"

namespace mec {
namespace road_clearance {

/**
 * An abstract factory class to switch between different road clearance modules.
 *
 * This class has to be implemented within a road clearance module library.
 */
class RoadClearanceModuleFactory {

public:

	/**
	 * RoadClearanceModuleFactory Destructor.
	 */
	virtual ~RoadClearanceModuleFactory() {}

	/**
	 * Create operation to be able to create new instances of a road clearance module.
	 *
	 * @param listener A weak reference to the RoadClearanceListener.
	 * @param input_queue A weak reference to the input queue.
	 * @param file_path The path to the road clearance configuration file.
	 *
	 * @return A shared pointer to a new road clearance module instance for use as extension.
	 */
	virtual std::shared_ptr<mec::extension::Extension> Create(std::weak_ptr<mec::environment::RoadClearanceListener> listener,
			std::weak_ptr<mec::environment::QueueListener<RoadClearanceInput>> input_queue,
			std::string file_path = "/etc/MECViewServer/road_clearance.json") = 0;

};

} /* end of namespace algorithm */
} /* end of namespace mec */

#endif /* ROAD_CLEARANCE_ROAD_CLEARANCE_MODULE_FACTORY_H_ */

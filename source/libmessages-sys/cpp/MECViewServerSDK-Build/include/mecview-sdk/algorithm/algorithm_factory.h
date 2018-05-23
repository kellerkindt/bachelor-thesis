/*
 * algorithm_factory.h
 *
 *  Created on: Sep 19, 2017
 *      Author: Niranjan Selvaraj
 *      Email: niranjan.selvaraj@it-designers.de
 */

#ifndef ALGORITHM_ALGORITHM_FACTORY_H_
#define ALGORITHM_ALGORITHM_FACTORY_H_

#include <memory>

#include "../extension/extension.h"
#include "../extension/queue_listener.h"

#include "../proto/SensorFrame.h"

#include "environment_listener.h"

namespace mec {
namespace algorithm {

/**
 * An abstract factory class to switch between different algorithms.
 *
 * This interface must be implemented in an algorithm library.
 */
class AlgorithmFactory {

public:

	/**
	 * AlgorithmFactory Destructor
	 */
	virtual ~AlgorithmFactory() {}

	/**
	 * Create operation to be able to create new instances of an algorithm.
	 *
	 * @param listener A reference to the EnvironmentListener. This listener is used to publish the algorithms output.
	 * @param frame_queue A referene to the input queue for the algorithm. All received sensor frame are passed to the algorithm using this queue.
	 * @param file_path The path to the algorithm configuration file. This configuration file can be parsed in the algorithms implementation of the mec::extension::Extension::ReadParameters operation.
	 *
	 * @return A shared pointer to a new algorithm instance for use as extension.
	 */
	virtual std::shared_ptr<mec::extension::Extension> Create(std::weak_ptr<mec::environment::EnvironmentListener> listener,
			std::weak_ptr<mec::environment::QueueListener<SensorFrame_t>> frame_queue,
			std::string file_path = "/etc/MECViewServer/algo.json");

};

} /* end of namespace algorithm */
} /* end of namespace mec */

#endif /* ALGORITHM_ALGORITHM_FACTORY_H_ */

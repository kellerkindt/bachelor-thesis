/*
 * sample_algorithm_factory.cpp
 *
 *  Created on: Sep 19, 2017
 *      Author: Niranjan Selvaraj
 *      Email: niranjan.selvaraj@it-designers.de
 */

#include "sample_algorithm.h"
#include <mecview-sdk/algorithm/algorithm_factory.h>

namespace mec {
namespace algorithm {

std::shared_ptr<mec::extension::Extension> AlgorithmFactory::Create(std::weak_ptr<mec::environment::EnvironmentListener> listener,
		std::weak_ptr<mec::environment::QueueListener<SensorFrame_t>> frame_queue,
		std::string file_path) {

	return std::make_shared<SampleAlgorithm>(listener, frame_queue, file_path);
}

} /* end of namespace algorithm */
} /* end of namespace mec */

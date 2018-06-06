
#ifndef ROAD_CLEARANCE_ROAD_CLEARANCE_LISTENER_H
#define ROAD_CLEARANCE_ROAD_CLEARANCE_LISTENER_H

#include <memory>
#include <ctime>

#include "../proto/RoadClearanceFrame.h"

namespace mec {
namespace environment {

/**
 * The road clearance listener class.
 */
class RoadClearanceListener {

public:

	/**
	 * The road clearance listener destructor.
	 */
	virtual ~RoadClearanceListener() {}

	/**
	 * An update method for road clearance frames.
	 *
	 * @param frame The road clearance information frame.
	 * @param time_reg Timestamp for performance measurements.
	 */
	virtual void Update(std::shared_ptr<RoadClearanceFrame_t> frame, const struct timespec* time_reg = nullptr) = 0;
};
} /* end of namespace environment */
} /* end of namespace mec */


#endif /* ROAD_CLEARANCE_ROAD_CLEARANCE_LISTENER_H */

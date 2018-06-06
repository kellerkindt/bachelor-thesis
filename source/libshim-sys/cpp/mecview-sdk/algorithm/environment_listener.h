#ifndef MECVIEWSERVER_ENVIRONMENTLISTENER_H
#define MECVIEWSERVER_ENVIRONMENTLISTENER_H

#include <memory>
#include <ctime>

#include "../proto/EnvironmentFrame.h"
#include "../proto/InitMessage.h"

namespace mec {
namespace environment {

/**
 * The environment model listener class.
 */
class EnvironmentListener {

public:

	/**
	 * The environmental listener destructor.
	 */
	virtual ~EnvironmentListener() {
	}

	/**
	 * An update method for the environment model.
	 *
	 * @param frame An environment frame information.
	 * @param time_reg Timestamp for performance measurements.
	 */
	virtual void Update(std::shared_ptr<EnvironmentFrame_t> frame,
			const struct timespec* time_reg = nullptr) = 0;

	/**
	 * Init operation for an environment listener.
	 *
	 * @param init_message The init message containing the sectors of the server.
	 */
	virtual void Init(std::shared_ptr<InitMessage_t> init_message) = 0;
};
} /* end of namespace environment */
} /* end of namespace mec */

#endif /* MECVIEWSERVER_ENVIRONMENTLISTENER_H */

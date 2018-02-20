/*
 * algorithm.h
 *
 *  Created on: Jul 10, 2017
 *      Author: spock
 */

#ifndef EXTENSION_EXTENSION_H_
#define EXTENSION_EXTENSION_H_

#include <memory>

#include "extension_parameters.h"

namespace mec{
namespace extension {

/**
 * Interface for MECViewServer extensions.
 */
class Extension : public ExtensionParameters {

public:

	/**
	 * Extension destructor.
	 */
	virtual ~Extension() {}

	/**
	 * Run Extension.
	 */
	virtual void Run() = 0;

	/**
	 * The MEC-Server calls the stop method as a part of shutdown process.
	 */
	virtual void Stop() = 0;

	/**
	 * Method to read extension parameters from the config path.
	 */
	virtual void ReadParameters() = 0;

	/**
	 * Getter for the version of the extension.
	 */
	virtual std::string GetVersion() = 0;

};

} /* end of namespace extension */
} /* end of namespace mec */

#endif /* EXTENSION_EXTENSION_H_ */

/*
 * algorithm_parameters.h
 *
 *  Created on: Sep 11, 2017
 *      Author: Niranjan Selvaraj
 *		Email: niranjan.selvaraj@it-designers.de
 */

#ifndef EXTENSION_EXTENSION_PARAMETERS_H_
#define EXTENSION_EXTENSION_PARAMETERS_H_

namespace mec{
namespace extension {

/**
 * An interface to read the parameters of the extension, which is read
 * from a configuration file.
 */
class ExtensionParameters {

public:

	/**
	 * Destructor method.
	 */
	virtual ~ExtensionParameters() {}

	/**
	 * Interface method to read the algorithm parameter from the configuration file.
	 */
	virtual void ReadParameters() = 0;
};

} /* end of namespace extension */
} /* end of namespace mec */

#endif /* EXTENSION_EXTENSION_PARAMETERS_H_ */

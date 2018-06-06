/*
 * TestDataProvider.h
 *
 *  Created on: Sep 26, 2017
 *      Author: hannes
 */

#ifndef SRC_ENVIRONMENT_FRAME_PROVIDER_H_
#define SRC_ENVIRONMENT_FRAME_PROVIDER_H_

#include <memory>

#include <mecview-sdk/proto/EnvironmentFrame.h>

namespace mec {
namespace algorithm {

/**
 * Testdata provider class for environment frames.
 *
 * This class can provide a hardcoded default environment frame and a configurable environment frame.
 * A file path to a XER encoded XML file can be passed to the class and the test environment frame
 * is constructed out of the given values.
 */
class EnvironmentFrameProvider {
public:
	/**
	 * The constructor.
	 */
	EnvironmentFrameProvider();

	/**
	 * The destructor.
	 */
	virtual ~EnvironmentFrameProvider();

	/**
	 * Setter for the XML file containing test data values.
	 *
	 * @param new_file_path The file path to the test XML file.
	 */
	void SetMessageFilePath(std::string new_file_path);

	/**
	 * Getter for the hardcoded default ASN.1 environment frame.
	 *
	 * @return Filled environment frame.
	 */
	std::shared_ptr<EnvironmentFrame_t> GetDefaultASN1Frame();

	/**
	 * Getter for the parsed ASN.1 environment frame.
	 *
	 * If no file is given, the default ASN.1 environment frame will be returned.
	 *
	 * @return Filled environment frame.
	 */
	std::shared_ptr<EnvironmentFrame_t> GetASN1Frame();

private:
	EnvironmentFrameProvider(const EnvironmentFrameProvider&) :
			file_path(""), test_data_ready(false) {
	}

	void Init(std::string new_file_path);

	void CreateDefaultASN1Frame();

	bool CreateASN1Frame(std::string new_file_path);

	std::string file_path;
	bool test_data_ready;
	std::shared_ptr<EnvironmentFrame_t> asn1_frame;
	std::shared_ptr<EnvironmentFrame_t> default_asn1_frame;
};

} /* namespace util */
} /* namespace mec */

#endif /* SRC_ENVIRONMENT_FRAME_PROVIDER_H_ */

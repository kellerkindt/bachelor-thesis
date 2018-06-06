/*
 * algorithm.h
 *
 *  Created on: Jul 10, 2017
 *      Author: Niranjan Selvaraj
 *      Email: niranjan.selvaraj@it-designers.de
 */

#ifndef SRC_SERVER_ENVIRONMENT_SAMPLE_ALGORITHM_ALGORITHM_H_
#define SRC_SERVER_ENVIRONMENT_SAMPLE_ALGORITHM_ALGORITHM_H_

#include <memory>
#include <thread>
#include <mutex>
#include <condition_variable>

#include <mecview-sdk/extension/extension.h>
#include <mecview-sdk/extension/queue_listener.h>
#include <mecview-sdk/proto/SensorFrame.h>
#include <mecview-sdk/algorithm/environment_listener.h>

#include "environment_frame_provider.h"

namespace mec {
namespace algorithm {

/**
 * The Algorithm is the main class that builds up the environment model based on the
 * received information from the sensor.
 */
class SampleAlgorithm: public mec::extension::Extension {

public:
	/**
	 * The Algorithm constructor.
	 * @param listener A reference to the environment listener.
	 * @param frame_queue A reference to the frame queue required by the Algorithm.
	 * @param file_path The path to the Algorithm configurations file.
	 */
	SampleAlgorithm(
			std::weak_ptr<mec::environment::EnvironmentListener> listener,
			std::weak_ptr<mec::environment::QueueListener<SensorFrame_t>> frame_queue,
			std::string file_path = "/etc/MECViewServer/algo.json");

	/**
	 * Algorithm destructor.
	 */
	virtual ~SampleAlgorithm();

	/**
	 * Initialise Algorithm.
	 */
	virtual void Init();

	/**
	 * Run Algorithm.
	 */
	virtual void Run();

	/**
	 * The MEC-Server calls the stop method as a part of shutdown process. This stops all active sessions
	 * registered and the associated IO service.
	 *
	 * This is part of the clean shutdown  releasing all IO services being used by the MEC-Server and individual sessions.
	 */
	virtual void Stop();

	/**
	 * Method to read algorithm parameters from the algo config path.
	 */
	virtual void ReadParameters();

	/**
	 * Getter for the version of the sample algorithm.
	 */
	virtual std::string GetVersion();

private:

	/**
	 * Transmits test data with a configurable send delay.
	 *
	 * This can not be used for measuring round trip times!
	 */
	void TransmitTestData();

	/**
	 * Operation for stopping the test data transmission.
	 */
	void StopTransmission();

	/**
	 * Operation to check if algorithm is still running.
	 */
	bool IsRunning();

	/**
	 * The environment listener reference.
	 */
	std::weak_ptr<mec::environment::EnvironmentListener> environment_listener;

	/**
	 * An queue that contains all incoming frames.
	 */
	std::weak_ptr<mec::environment::QueueListener<SensorFrame_t>> update_queue;

	/**
	 * A flag to check the status of the algorithm.
	 */
	bool running;

	/**
	 * Signals the current transmission state in case of a transmission loop.
	 */
	bool transmitting;

	/**
	 * Indicator for a received sensor::frame.
	 *
	 * Used for timeout detection in algorithm to stop test data transmission.
	 */
	bool receivedFrame;

	/**
	 * Transmission thread member.
	 */
	std::shared_ptr<std::thread> transmission_thread;

	/* *
	 * When common resources are accessed by different threads, mutex.
	 */
	std::mutex algo_lock;

	/**
	 * Condition variable for blocking behaviour of the pop operation.
	 */
	std::condition_variable condition_changed;

	/**
	 * filepath to the algorithm configuration file.
	 */
	std::string algo_config_path;

	/**
	 * Delay between sending environment model messages to the vehicle.
	 *
	 * If the value is 0, an environment frame is published after a sensor::frame is received.
	 * This can be used for round trip time measurement, the time stamp is copied from the
	 * sensor frame.
	 */
	int send_delay_ms;

	/**
	 * Timeout value for last received sensor message in milliseconds(ms).
	 */
	int sensor_data_timeout_ms;

	std::string environment_frame_file;

	EnvironmentFrameProvider data_provider;
};

}

}

#endif /* SRC_SERVER_ENVIRONMENT_SAMPLE_ALGORITHM_ALGORITHM_H_ */

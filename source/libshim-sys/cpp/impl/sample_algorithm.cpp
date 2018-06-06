/*
 * algorithm.cpp
 *
 *  Created on: Aug 1, 2017
 *      Author: Niranjan Selvaraj
 *		Email: niranjan.selvaraj@it-designers.de
 */

#include "sample_algorithm.h"

#include <chrono>
#include <memory>
#include <iostream>

#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>
#include <boost/foreach.hpp>

namespace mec {
namespace algorithm {

SampleAlgorithm::SampleAlgorithm(
		std::weak_ptr<mec::environment::EnvironmentListener> listener,
		std::weak_ptr<mec::environment::QueueListener<SensorFrame_t>> frame_queue,
		std::string file_path) :
		environment_listener(listener), update_queue(frame_queue), running(
				false), transmitting(false), receivedFrame(
				false), algo_config_path(file_path), send_delay_ms(100), sensor_data_timeout_ms(
				1000), environment_frame_file("")
				{
	this->ReadParameters();
	this->data_provider.SetMessageFilePath(this->environment_frame_file);
}

SampleAlgorithm::~SampleAlgorithm() {
}

void SampleAlgorithm::Init() {
	{
		std::unique_lock<std::mutex> lock(this->algo_lock);
		this->update_queue.lock()->SetStop(false);
		this->running = true;
	}
}

void SampleAlgorithm::Run() {

	bool started_transmission = false;

	while (this->IsRunning()) {

		std::shared_ptr<SensorFrame_t> message =
				this->update_queue.lock()->Pop();

//		// Using the Pop-Operation with Timeout.
//		std::shared_ptr<SensorFrame_t> message = nullptr;
//
//		while(message == nullptr && this->IsRunning()) {
//			message = this->update_queue.lock()->Pop(std::chrono::duration<int, std::milli>(100));
//		}

		if(!this->transmitting && started_transmission) {
			if(this->transmission_thread->joinable()) {
				this->transmission_thread->join();
			}
			started_transmission = false;
		}

		this->receivedFrame = true;

		if (message != nullptr) {

			if (this->send_delay_ms == 0) { // if no send delay is given, use the RTT measurement mode.

				std::shared_ptr<EnvironmentFrame_t> env_frame =
						data_provider.GetASN1Frame();

				// copy the sender id for roundtrip time measuerements...
				env_frame->envelope.server_id = message->envelope.sender_id;
				// copy the "hack" pole_id to the version_id (content is the message counter for identifying the message per client)
				env_frame->envelope.version = message->envelope.pole_id;

				// copy the timestamp for "two-way" round-trip time measurement (full processing chain!)
				env_frame->header.timestamp = message->header.timestamp;

				this->environment_listener.lock()->Update(env_frame);
			} else {
				if (started_transmission == false) {

					started_transmission = true;

					this->transmission_thread = std::make_shared<std::thread>(
							std::bind(&SampleAlgorithm::TransmitTestData,
									this));
				}
			}
		} else {
			//ToDo: check for alternate action sequence
		}
	}
}

void SampleAlgorithm::Stop() {

	std::unique_lock<std::mutex> lock(this->algo_lock);
	this->running = false;

	if (std::shared_ptr<mec::environment::QueueListener<SensorFrame_t>> shared_update_queue =
			this->update_queue.lock()) {
		shared_update_queue->SetStop(true);
	}

	if (this->transmission_thread && this->transmission_thread->joinable()) {
		this->transmission_thread->join();
	}
}

void SampleAlgorithm::StopTransmission() {
	std::unique_lock<std::mutex> lock(this->algo_lock);
	this->transmitting = false;
}

bool SampleAlgorithm::IsRunning() {
	std::unique_lock<std::mutex> lock(this->algo_lock);
	return (this->running);
}

void SampleAlgorithm::TransmitTestData() {
	int message_delay = 0;

	this->transmitting = true;

	while (this->running && this->transmitting) {

		if (message_delay >= (this->sensor_data_timeout_ms * 1000)) {
			message_delay = 0;

			if (!this->receivedFrame) {
				this->StopTransmission();
			}

			this->receivedFrame = false;
		}

		this->environment_listener.lock()->Update(data_provider.GetASN1Frame());

		message_delay += send_delay_ms * 1000;
		usleep(send_delay_ms * 1000);
	}
	this->transmitting = false;
}

void SampleAlgorithm::ReadParameters() {
	boost::property_tree::ptree tree;

	try {
		boost::property_tree::read_json(this->algo_config_path, tree);

		BOOST_FOREACH(boost::property_tree::ptree::value_type &v, tree.get_child("algorithm_parameters")) {

			std::string parameter_name = v.second.get<std::string>("name");

			if (parameter_name == "send_delay_ms") {
				this->send_delay_ms = v.second.get<int>("value");
			} else if (parameter_name == "sensor_timeout_ms") {
				this->sensor_data_timeout_ms = v.second.get<int>("value");
			} else if(parameter_name == "environment_frame_file") {
				this->environment_frame_file = v.second.get<std::string>("value");
			}

		}
	} catch (std::exception &ex) {
		std::cout << "SampleAlgorithm: Could not parse the configuration file "
				<< this->algo_config_path << ": " << ex.what() << std::endl;
	}
}

std::string SampleAlgorithm::GetVersion() {
	return "0.0.1";
}

}
}


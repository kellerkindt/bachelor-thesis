/*
 * QueueOperations.h
 *
 *  Created on: Sep 14, 2017
 *      Author: Niranjan Selvaraj
 *		Email: niranjan.selvaraj@it-designers.de
 */

#ifndef SRC_SERVER_ENVIRONMENT_QUEUE_LISTENER_H_
#define SRC_SERVER_ENVIRONMENT_QUEUE_LISTENER_H_

#include <memory>
#include <chrono>

namespace mec {
namespace environment {

template<typename T>
/**
 * A abstract listener to the sensor frame queue class
 * Follow the FIFO rule.
 */
class QueueListener {

public:

	/**
	 * The QueueListener destructor
	 */
	virtual ~QueueListener() {
	}

	/**
	 * Adding new sensor frames into the queue.
	 * @param sensor_frame An object detection frame from the sensor, received and decoded.
	 */
	virtual void Add(const std::shared_ptr<T> sensor_frame) = 0;

	/**
	 * Fetching the earliest object detection frame from the queue
	 * @return Object detection frame from the Sensor.
	 */
	virtual const std::shared_ptr<T> Pop() = 0;

	/**
	 * The take the message from the queue. Follows a first in first out pattern.
	 *
	 * @param timeout The timeout for the blocking wait. The operation will return after
	 * 				  timeout milliseconds, if no message is in the queue the return will be a nullptr.
	 *
	 * @return A reference to the protobuf message.
	 */
	virtual const std::shared_ptr<T> Pop(
			std::chrono::duration<int, std::milli> timeout) = 0;

	/**
	 * A function to check if the queue is empty
	 *
	 * @return true if queue is not empty, else return false.
	 */
	virtual bool MessageAvailable() = 0;

	/**
	 * Returns the current number of messages in the queue.
	 *
	 * @return the current number of messages in the queue.
	 */
	virtual int GetMessageCount() = 0;

	/**
	 * Sets the stop flag. An indication to stop data transfer to algorithm
	 * @param stop Stop flag.
	 */
	virtual void SetStop(bool stop) = 0;
};

}
}

#endif /* SRC_SERVER_ENVIRONMENT_QUEUE_LISTENER_H_ */

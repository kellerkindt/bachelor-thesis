#ifndef SRC_SERVER_ENVIRONMENT_UPDATE_QUEUE_H_
#define SRC_SERVER_ENVIRONMENT_UPDATE_QUEUE_H_

#include <deque>
#include <memory>
#include <condition_variable>
#include <mutex>
#include <chrono>

#include <iostream>

#include "../mecview-sdk/extension/queue_listener.h"

namespace mec {
    namespace environment {

/**
 * The UpdateQueue class is used to store and retrieve incoming messages.
 * A common queue is used to incoming messages from the sensors and
 * the stored messages are retrieved by the algorithm.
 */
        template<typename T>
        class UpdateQueue: public QueueListener<T> {

        private:

            /**
             * The common sensor frame queue that is to used by the algorithm.
             */
            std::deque<std::shared_ptr<T>> sensor_frame_queue;

            /* *
             * When common resources are accessed by different threads,
             * mutex @param message_queue_lock provides exclusivity and
             * does not allow multiple threads to vary data in the common resource at the same time.
             */
            std::mutex message_queue_lock;

            /**
             * Condition variable for blocking behaviour of the pop operation.
             */
            std::condition_variable message_available;

            /**
             * Used for immediate return in the pop operation if the queue is stopped.
             */
            bool stop;

        public:

            /**
             * The update queue constructor.
             */
            UpdateQueue() :
                    stop(false) {

            }

            /**
             * The update queue destructor.
             */
            virtual ~UpdateQueue() {

            }

            /**
             * To add the incoming messages into the sensor frame queue
             * @param sensor_frame A reference to the protobuf message.
             */
            void Add(const std::shared_ptr<T> sensor_frame) {
                {
                    std::unique_lock<std::mutex> lock(message_queue_lock);
                    // std::cout << "manager Queue::Add" << std::endl << std::flush;
                    sensor_frame_queue.push_back(sensor_frame);
                    while (sensor_frame_queue.size() > 100) {
                        sensor_frame_queue.pop_front();
                    }
                }
                message_available.notify_one();
            }

            /**
             * The take the message from the queue. Follows a first in first out pattern.
             *
             * @return A reference to the message, nullptr if no message was found.
             */

            const std::shared_ptr<T> Pop() {
                std::unique_lock<std::mutex> lock(message_queue_lock);
                // std::cout << "manager Queue::Pop" << std::endl << std::flush;

                if(this->sensor_frame_queue.size() <= 0) {
                    message_available.wait(lock,
                                           [&] {return (!this->sensor_frame_queue.empty() || this->stop);});
                }

                if (this->sensor_frame_queue.size() > 0) {
                    std::shared_ptr<T> message(this->sensor_frame_queue.front());
                    this->sensor_frame_queue.pop_front();
                    return message;
                } else {
                    return nullptr;
                }
            }

            /**
             * The take the message from the queue. Follows a first in first out pattern.
             *
             * @param timeout The timeout for the blocking wait. The operation will return after
             * 				  timeout milliseconds, if no message is in the queue the return will be a nullptr.
             *
             * @return A reference to the message, nullptr if no message was found.
             */
            const std::shared_ptr<T> Pop(std::chrono::duration<int,std::milli> timeout) {

                std::unique_lock<std::mutex> lock(message_queue_lock);

                if(this->sensor_frame_queue.size() <= 0) {
                    message_available.wait_for(lock, timeout, [&] {return (!this->sensor_frame_queue.empty() || this->stop);});
                }

                if(this->sensor_frame_queue.size() > 0) {
                    std::shared_ptr<T> message(this->sensor_frame_queue.front());
                    this->sensor_frame_queue.pop_front();
                    return message;
                } else {
                    return nullptr;
                }
            }

            /**
             * Indicates the availability of at least one message in the queue.
             */
            bool MessageAvailable() {
                std::unique_lock<std::mutex> lock(message_queue_lock);
                return (this->sensor_frame_queue.size() > 0);
            }

            int GetMessageCount() {
                std::unique_lock<std::mutex> lock(message_queue_lock);
                return this->sensor_frame_queue.size();
            }

            /**
             * Sets the stop indicator for the message queue.
             * @param stop A flag to stop updating the queue.
             *
             * If stop is set, the blocking pop operation will return.
             */
            void SetStop(bool stop) {
                std::unique_lock<std::mutex> lock(message_queue_lock);
                this->stop = stop;
                message_available.notify_all();
            }

        };

    }
/* end of namespace environment */
} /* end of namespace mec */

#endif /* SRC_SERVER_ENVIRONMENT_UPDATE_QUEUE_H_ */
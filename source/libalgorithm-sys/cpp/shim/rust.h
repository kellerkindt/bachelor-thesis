#include <iostream>
#include <memory>
#include <string>
#include <thread>

#include "../mecview-sdk/proto/EnvironmentFrame.h"
#include "../mecview-sdk/proto/InitMessage.h"
#include "../mecview-sdk/proto/SensorFrame.h"
#include "../mecview-sdk/algorithm/algorithm_factory.h"
#include "../mecview-sdk/algorithm/environment_listener.h"
#include "../mecview-sdk/extension/queue_listener.h"
#include "../mecview-sdk/extension/extension.h"

#include "update_queue.h"


typedef void* RustInstance;
struct RustShim;

extern "C" {
    RustShim* shim_create(char *config_file);
    void shim_destroy(RustShim *shim);
    void shim_send_sensor_frame(RustShim *shim, SensorFrame_t *frame);
    void shim_publish_environment_frame(RustShim *shim, EnvironmentFrame_t *frame);
    void shim_publish_init_message(RustShim *shim, InitMessage_t *message);
}


class RustShimInternal : public std::enable_shared_from_this<RustShimInternal>, public mec::environment::EnvironmentListener {
private:
    std::shared_ptr<mec::environment::QueueListener<SensorFrame_t>> queue_listener;
    std::shared_ptr<mec::extension::Extension> algorithm;
    std::shared_ptr<std::thread> algorithm_thread;
    RustShim* shim;

public:
    RustShimInternal(RustShim* shim) {
        // std::cout <<"RustShimInternal setting shim" << std::endl << std::flush;
        this->shim = shim;
        // std::cout <<"RustShimInternal creating queue_listener" << std::endl << std::flush;
        this->queue_listener = std::make_shared<mec::environment::UpdateQueue<SensorFrame_t>>();
        // std::cout <<"RustShimInternal done" << std::endl << std::flush;
    }

    ~RustShimInternal() {
        if (this->algorithm) {
            this->algorithm->Stop();
            this->algorithm.reset();
        }
        if (this->algorithm_thread) {
            this->algorithm_thread->join();
            this->algorithm_thread.reset();
        }
    }

    bool init_algorithm(std::string config_file) {
        // std::cout <<"init_algorithm creating factory" << std::endl << std::flush;
        auto factory = mec::algorithm::AlgorithmFactory();
        // std::cout <<"init_algorithm creating instance with config from " << config_file << std::endl << std::flush;
        this->algorithm = factory.Create(
                this->shared_from_this(),
                queue_listener,
                config_file
        );
        if(this->algorithm) {
            this->algorithm->Init();
            this->algorithm_thread = std::make_shared<std::thread>(std::bind(&mec::extension::Extension::Run, this->algorithm));
            return true;
        } else {
            // std::cout <<"init_algorithm algorithm invalid " << std::endl << std::flush;
            return false;
        }
    }

    std::shared_ptr<RustShimInternal> ptr() {
        return this->shared_from_this();
    }

    void send_sensor_frame(SensorFrame_t* frame) {
        // std::cout <<"RustShimInternal adding SensorFrame" << std::endl << std::flush;
        this->queue_listener->Add(std::shared_ptr<SensorFrame_t>(frame, [](SensorFrame_t* f){
            asn_DEF_SensorFrame.free_struct(&asn_DEF_SensorFrame, f, 0);
        }));
    }



    /**
     * An update method for the environment model.
     *
     * @param frame An environment frame information.
     * @param time_reg Timestamp for performance measurements.
     */
    void Update(std::shared_ptr<EnvironmentFrame_t> frame, const struct timespec*) {
        // std::cout <<"RustShimInternal Update, time_reg=" << time_reg << std::endl;
        shim_publish_environment_frame(this->shim, frame.get());
    }

    /**
     * Init operation for an environment listener.
     *
     * @param init_message The init message containing the sectors of the server.
     */
    void Init(std::shared_ptr<InitMessage_t> init_message) {
        // std::cout <<"RustShimInternal Init" << std::endl;
        shim_publish_init_message(this->shim, init_message.get());
    }
};

struct RustShim {
    void (*publish_environment_frame)(RustInstance*, EnvironmentFrame_t*);
    void (*publish_init_message)(RustInstance*, InitMessage_t*);
    RustInstance* instance;
    std::shared_ptr<RustShimInternal> internal;
};


RustShim* shim_create(char* config_file) {
    RustShim* shim = (RustShim*) calloc(1, sizeof(RustShim));
    // std::cout <<"calloc" << std::endl;
    if (shim != NULL) {
        // std::cout <<"succeeded" << std::endl << std::flush;
        shim->internal = std::make_shared<RustShimInternal>(shim);
        // std::cout <<"init_algorithm" << std::endl << std::flush;
        if (!shim->internal->init_algorithm(std::string(config_file))) {
            shim_destroy(shim);
            shim = NULL;
        }
    }
    return shim;
}

void shim_send_sensor_frame(RustShim* shim, SensorFrame_t* frame) {
    if (shim != NULL) {
        shim->internal->send_sensor_frame(frame);
    }
}

void shim_publish_environment_frame(RustShim* shim, EnvironmentFrame_t* frame) {
    // std::cout <<"shim_publish_environment_frame" << std::endl;
    if (shim != NULL && shim->instance != NULL && shim->publish_environment_frame != NULL) {
        // std::cout <<"shim_publish_environment_frame calling callback" << std::endl;
        shim->publish_environment_frame(shim->instance, frame);
    }
}

void shim_publish_init_message(RustShim* shim, InitMessage_t* message) {
    // std::cout <<"shim_publish_init_message" << std::endl;
    if (shim != NULL && shim->instance != NULL && shim->publish_init_message != NULL) {
        // std::cout <<"shim_publish_init_message calling callback" << std::endl;
        shim->publish_init_message(shim->instance, message);
    }
}

void shim_destroy(RustShim* shim) {
    if (shim != NULL) {
        shim->internal.reset();
        free(shim);
    }
}
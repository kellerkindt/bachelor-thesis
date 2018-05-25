#include <memory>
#include <string>

#include "../mecview-sdk/proto/EnvironmentFrame.h"
#include "../mecview-sdk/proto/InitMessage.h"
#include "../mecview-sdk/proto/SensorFrame.h"
#include "../mecview-sdk/algorithm/algorithm_factory.h"
#include "../mecview-sdk/algorithm/environment_listener.h"
#include "../mecview-sdk/extension/queue_listener.h"
#include "../mecview-sdk/extension/extension.h"

#include "update_queue.h"


typedef void* RustEventListener;
struct RustShim;

void publish_environment_frame(RustShim* shim, EnvironmentFrame_t* frame);
void publish_init_message(RustShim* shim, InitMessage_t* message);


class RustShimInternal : public mec::environment::EnvironmentListener, public std::enable_shared_from_this<RustShimInternal> {
private:
    std::shared_ptr<mec::environment::QueueListener<SensorFrame_t>> queue_listener;
    std::shared_ptr<mec::extension::Extension> algorithm;
    RustShim* shim;

public:
    RustShimInternal(std::string config_file, RustShim* shim) : shim(shim), queue_listener(std::make_shared<mec::environment::UpdateQueue<SensorFrame_t>>()) {
        auto factory = mec::algorithm::AlgorithmFactory();
        algorithm = factory.Create(
                shared_from_this(),
                queue_listener,
                config_file
        );
    }

    ~RustShimInternal() {

    }

    void send_sensor_frame(SensorFrame_t* frame) {
        this->queue_listener->Add(std::shared_ptr<SensorFrame_t>(frame, [](SensorFrame_t* f){
            asn_DEF_SensorFrame.free_struct(&asn_DEF_SensorFrame, f, 1);
        }));
    }



    /**
     * An update method for the environment model.
     *
     * @param frame An environment frame information.
     * @param time_reg Timestamp for performance measurements.
     */
    void Update(std::shared_ptr<EnvironmentFrame_t> frame,
                const struct timespec* time_reg = nullptr) {
        // keeper leaks the content
        std::shared_ptr<EnvironmentFrame_t> keeper((EnvironmentFrame_t*) calloc(1, sizeof(EnvironmentFrame_t)), [](EnvironmentFrame_t*){});
        frame.swap(keeper); // frame is now empty, keeper holds the frame
        EnvironmentFrame_t* raw = keeper.get();
        publish_environment_frame(this->shim, raw);
    }

    /**
     * Init operation for an environment listener.
     *
     * @param init_message The init message containing the sectors of the server.
     */
    void Init(std::shared_ptr<InitMessage_t> init_message) {
        // keeper leaks the content
        std::shared_ptr<InitMessage_t> keeper((InitMessage_t*) calloc(1, sizeof(InitMessage_t)), [](InitMessage_t*){});
        init_message.swap(keeper); // frame is now empty, keeper holds the frame
        InitMessage_t* raw = keeper.get();
        publish_init_message(this->shim, raw);
    }
};

struct RustShim {
    void (*publish_environment_frame)(RustEventListener*, EnvironmentFrame_t*);
    void (*publish_init_message)(RustEventListener*, InitMessage_t*);
    std::shared_ptr<RustShimInternal> internal;
    RustEventListener* eventListener;
};


RustShim* create_rust_shim_for_algorithm(char* config_file) {
    RustShim* shim = (RustShim*) calloc(1, sizeof(RustShim));
    if (shim != nullptr) {
        shim->internal = std::make_shared<RustShimInternal>(config_file, shim);
    }
    return shim;
}

void send_sensor_frame(RustShim* shim, SensorFrame_t* frame) {
    if (shim != nullptr) {
        shim->internal->send_sensor_frame(frame);
    }
}

void publish_environment_frame(RustShim* shim, EnvironmentFrame_t* frame) {
    if (shim != nullptr && shim->eventListener != nullptr) {
        shim->publish_environment_frame(shim->eventListener, frame);
    }
}

void publish_init_message(RustShim* shim, InitMessage_t* message) {
    if (shim != nullptr && shim->eventListener != nullptr) {
        shim->publish_init_message(shim->eventListener, message);
    }
}
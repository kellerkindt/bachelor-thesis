#[macro_use]
extern crate log;

extern crate libalgorithm;
extern crate libasn;
extern crate libmessages;

use std::fmt::Debug;
use std::sync::Arc;

use libasn::raw;
use libasn::raw::EnvironmentFrame;
use libasn::raw::EnvironmentObjectDetection;
use libasn::raw::MovingVector;
use libasn::raw::PathPoint;
use libasn::raw::SensorFrame;
use libasn::AsnMessage;

use libmessages::RawMessage;

use libalgorithm::Algorithm;
use libalgorithm::CountListener;
use libalgorithm::EnvironmentListener;
use libalgorithm::ListenerManager;

pub struct SampleAlgorithm<I: Send + Debug + PartialEq + 'static> {
    manager: ListenerManager<SensorFrame, I>,
    environment_frame: Option<Box<EnvironmentFrame>>,
}

impl<I: Send + Debug + PartialEq + 'static> Default for SampleAlgorithm<I> {
    fn default() -> Self {
        SampleAlgorithm {
            manager: ListenerManager::default(),
            environment_frame: Default::default(),
        }
    }
}

impl<I: Send + Debug + PartialEq + 'static> SampleAlgorithm<I> {
    fn environment_model(&mut self, frame: &SensorFrame) -> Arc<RawMessage> {
        trace!("Going to create new RawMessage");
        let encoded = if let Some(ref mut env) = self.environment_frame {
            trace!(
                "Updating existing with timestamp: {}",
                frame.header.timestamp
            );
            Self::update_for_roundtrip_measurement(env, frame);
            trace!("Going to encode");
            env.try_encode_uper()
        } else {
            trace!("Create new instance");
            let mut env = Box::new(EnvironmentFrame::default());
            trace!("Populating frame");
            Self::populate_default_frame(&mut env);
            Self::update_for_roundtrip_measurement(&mut env, frame);
            trace!("Going to encode");
            let encoded = env.try_encode_uper();
            self.environment_frame = Some(env);
            encoded
        };

        trace!("Checking encode result: {:?}", encoded);
        match encoded {
            Ok(raw) => Arc::new(raw),
            Err(e) => {
                error!("Failed to encode environment_model: {:?}", e);
                panic!("Failed to encode environment_model: {:?}", e);
            }
        }
    }

    fn update_for_roundtrip_measurement(env: &mut EnvironmentFrame, sensor: &SensorFrame) {
        env.header.timestamp = sensor.header.timestamp;
        env.envelope.server_id = sensor.envelope.sender_id;
        env.envelope.version = sensor.envelope.pole_id;
    }

    fn populate_default_frame(env: &mut EnvironmentFrame) {
        // See 'environment_frame_provider.cpp' EnvironmentFrameProvider::CreateDefaultASN1Frame()

        env.envelope.version = 1;
        env.envelope.server_id = 0;

        env.envelope.reference_point.latitude = 484010822;
        env.envelope.reference_point.longitude = 99876076;
        env.envelope.reference_point.altitude = 256000;

        for object_count in 0..15 {
            let mut object_detection =
                unsafe { Box::from_raw(raw::zeroed::<EnvironmentObjectDetection>()) };

            object_detection.global_id = object_count;
            object_detection.probability_of_existence = 150;

            object_detection.position_offset.position_north = 256;
            object_detection.position_offset.std_dev_position_north = unsafe { raw::alloc(1) };

            object_detection.position_offset.position_east = 5000;
            object_detection.position_offset.std_dev_position_east = unsafe { raw::alloc(1) };

            let mut moving_vector = unsafe { Box::from_raw(raw::zeroed::<MovingVector>()) };

            moving_vector.velocity_north = 80;
            moving_vector.std_dev_velocity_north = unsafe { raw::alloc(5) };

            moving_vector.velocity_east = 266;
            moving_vector.std_dev_velocity_east = unsafe { raw::alloc(5) };

            object_detection.moving_vector = Box::into_raw(moving_vector);

            object_detection.type_ = raw::ObjectType_ObjectType_car as raw::ObjectType_t;
            object_detection.type_probability = 190;

            // SIZE

            object_detection.size.length = 450;
            object_detection.size.std_dev_length = unsafe { raw::alloc(1) }; // optional

            object_detection.size.width = unsafe { raw::alloc(230) }; // optional
            object_detection.size.std_dev_width = unsafe { raw::alloc(1) }; // optional

            object_detection.size.height = unsafe { raw::alloc(180) }; // optional
            object_detection.size.std_dev_height = unsafe { raw::alloc(1) }; // optional

            // COVARIANCE

            object_detection.covariance.north_pos_east_pos = unsafe { raw::alloc(1_000_000) };
            object_detection.covariance.north_pos_north_vel = unsafe { raw::alloc(1_000) };
            object_detection.covariance.north_pos_east_vel = unsafe { raw::alloc(1_000) };

            object_detection.covariance.east_pos_north_vel = unsafe { raw::alloc(1_000) };
            object_detection.covariance.east_pos_east_vel = unsafe { raw::alloc(1_000) };
            object_detection.covariance.north_vel_east_vel = unsafe { raw::alloc(100) };

            object_detection.orientation = 25;
            object_detection.std_dev_orientation = 950;
            object_detection.measured = 1;

            // PREDICTED PATH

            object_detection.predicted_path.timestamp_dt = 10;

            for _ in 0..10 {
                let mut pathpoint = unsafe { Box::from_raw(raw::zeroed::<PathPoint>()) };

                pathpoint.position_offset.position_north = 256;
                pathpoint.position_offset.std_dev_position_north = unsafe { raw::alloc(1) };

                pathpoint.position_offset.position_east = 5_000;
                pathpoint.position_offset.std_dev_position_east = unsafe { raw::alloc(1) };

                pathpoint.probability = 195;

                let result = unsafe {
                    raw::asn_set_add(
                        &mut object_detection.predicted_path.path as *mut _
                            as *mut ::std::os::raw::c_void,
                        Box::into_raw(pathpoint) as *mut ::std::os::raw::c_void,
                    )
                };

                if result != 0 {
                    error!("asn_set_add() failed to add PathPoint");
                    panic!("asn_set_add() failed to add PathPoint");
                }
            }

            let result = unsafe {
                raw::asn_set_add(
                    &mut env.object_detections as *mut _ as *mut ::std::os::raw::c_void,
                    Box::into_raw(object_detection) as *mut ::std::os::raw::c_void,
                )
            };

            if result != 0 {
                error!("asn_set_add() failed to add ObjectDetection");
                panic!("asn_set_add() failed to add ObjectDetection");
            }
        }
    }
}

impl<I: Send + Debug + PartialEq + 'static> Algorithm<SensorFrame, I> for SampleAlgorithm<I> {
    fn update(&mut self, update: Box<SensorFrame>) {
        let model = self.environment_model(&update);
        self.manager.publish_environment_model(model);
    }

    fn publish(&mut self, model: RawMessage) {
        self.manager.publish_environment_model(Arc::new(model));
    }

    fn subscribe_environment_model(&mut self, identifier: I, listener: EnvironmentListener) {
        self.manager.add_environment_listener(identifier, listener);
    }

    fn unsubscribe_environment_model(&mut self, identifier: I) {
        self.manager.remove_environment_listener(identifier);
    }

    fn activate_environment_model_subscription(&mut self, identifier: I) {
        self.manager.activate_environment_model_listener(identifier);
    }

    fn deactivate_environment_model_subscription(&mut self, identifier: I) {
        self.manager
            .deactivate_environment_model_listener(identifier);
    }

    fn subscribe_listener_count(&mut self, identifier: I, listener: CountListener) {
        self.manager.add_count_listener(identifier, listener);
    }

    fn unsubscribe_listener_count(&mut self, identifier: I) {
        self.manager.remove_count_listener(identifier);
    }
}

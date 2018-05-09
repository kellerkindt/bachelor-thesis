use std::io::Error;
use std::net::SocketAddr;
use std::ops::IndexMut;
use std::sync::Arc;

use messages::asn::raw;
use messages::asn::raw::EnvironmentFrame;
use messages::asn::raw::EnvironmentObjectDetection;
use messages::asn::raw::MovingVector;
use messages::asn::raw::PathPoint;
use messages::asn::raw::SensorFrame;
use messages::asn::AsnMessage;
use messages::RawMessage;

use super::Algorithm;
use super::CountListener;
use super::EnvironmentListener;

#[derive(Default)]
pub struct SampleAlgorithm {
    model_listener: Vec<(SocketAddr, bool, EnvironmentListener<EnvironmentFrame>)>,
    count_listener: Vec<(SocketAddr, CountListener)>,
    environment_frame: Option<Box<EnvironmentFrame>>,
}

impl SampleAlgorithm {
    pub fn set_environment_frame(&mut self, frame: Option<Box<EnvironmentFrame>>) {
        self.environment_frame = frame;
    }

    fn push_model_listener(&mut self, id: SocketAddr, sink: EnvironmentListener<EnvironmentFrame>) {
        trace!("Adding model listener with id={}", id);
        let len = self.model_listener.len();
        self.model_listener.push((id, false, sink));

        trace!(
            "Now subscribed model listeners: {}",
            self.model_listener.len()
        );

        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn remove_model_listener(&mut self, remove: SocketAddr) {
        trace!("Removing model listener with id={}", remove);
        let len = self.model_listener.len();
        self.model_listener.retain(|(id, _, _)| id.ne(&remove));
        trace!(
            "Removed {} matches, still subscribed: {}",
            len - self.model_listener.len(),
            self.model_listener.len()
        );
        if len != self.model_listener.len() {
            self.on_model_count_changed();
        }
    }

    fn activate_model_listener(&mut self, id: SocketAddr) {
        self.model_listener
            .iter_mut()
            .filter(|(addr, _, _)| addr.eq(&id))
            .for_each(|(_, ref mut active, _)| *active = true);
    }

    fn deactivate_model_listener(&mut self, id: SocketAddr) {
        self.model_listener
            .iter_mut()
            .filter(|(addr, _, _)| addr.eq(&id))
            .for_each(|(_, ref mut active, _)| *active = false);
    }

    fn push_count_listener(&mut self, id: SocketAddr, mut sink: CountListener) {
        trace!("Adding count listener with id={}", id);
        if sink(self.model_listener.len()).is_ok() {
            self.count_listener.push((id, sink));
        }
        trace!(
            "Now subscribed count listeners: {}",
            self.count_listener.len()
        );
    }

    fn remove_count_listener(&mut self, remove: SocketAddr) {
        trace!("Removing count listener with id={}", remove);
        self.count_listener.retain(|(id, _)| id.ne(&remove));
        trace!(
            "Still subscribed count listeners. {}",
            self.count_listener.len()
        );
    }

    fn on_update(&mut self, frame: &SensorFrame) {
        trace!("Sensor update received: {:?}", frame);
        let len = self.model_listener.len();

        if len > 0 {
            let env = self.environment_model(frame);

            Self::retain_mut(&mut self.model_listener, |(addr, active, listener)| {
                trace!("Sending model to ModelListener/{}", addr);
                !*active || listener(env.clone()).is_ok()
            });

            if len != self.model_listener.len() {
                self.on_model_count_changed();
            }
        }
    }

    fn on_model_count_changed(&mut self) {
        let count = self.model_listener.len();
        trace!("on_model_count_changed, current count: {}", count);
        Self::retain_mut(&mut self.count_listener, |(_, listener)| {
            listener(count).is_ok()
        });
    }

    fn retain_mut<T, F: Fn(&mut T) -> bool>(vec: &mut Vec<T>, keep: F) {
        // nightly: see Vec::drain_filter
        // Vec::retain only provides &T and not &mut T :(
        let mut index = 0_usize;
        for _ in 0..vec.len() {
            if !keep(vec.index_mut(index)) {
                vec.swap_remove(index);
            } else {
                index += 1;
            }
        }
    }

    fn environment_model(&mut self, frame: &SensorFrame) -> Arc<RawMessage<EnvironmentFrame>> {
        trace!("Going to create new RawMessage<EnvironmentFrame>");
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
                Box::new(unsafe { ::std::mem::zeroed::<EnvironmentObjectDetection>() });

            object_detection.global_id = object_count;
            object_detection.probability_of_existence = 75_000;

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
            object_detection.type_probability = 95_000;

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

                pathpoint.probybility = 195;

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

impl Algorithm<SensorFrame, EnvironmentFrame> for SampleAlgorithm {
    type Identifier = SocketAddr;

    fn update(&mut self, update: Box<SensorFrame>) -> Result<(), Error> {
        self.on_update(&update);
        Ok(())
    }

    fn subscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
        sink: EnvironmentListener<EnvironmentFrame>,
    ) -> Result<(), Error> {
        self.push_model_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_environment_model(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_model_listener(identifier);
        Ok(())
    }

    fn activate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.activate_model_listener(identifier);
        Ok(())
    }

    fn deactivate_environment_model_subscription(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.deactivate_model_listener(identifier);
        Ok(())
    }

    fn subscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
        sink: CountListener,
    ) -> Result<(), Error> {
        self.push_count_listener(identifier, sink);
        Ok(())
    }

    fn unsubscribe_listener_count(
        &mut self,
        identifier: <Self as Algorithm<SensorFrame, EnvironmentFrame>>::Identifier,
    ) -> Result<(), Error> {
        self.remove_count_listener(identifier);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retain_mut_removes_correct() {
        let mut list = vec![1, 2, 3, 4, 5];
        SampleAlgorithm::retain_mut(&mut list, |i| *i % 2 == 0);
        assert_eq!(2, list.len());
        assert!(list.contains(&2));
        assert!(list.contains(&4));
    }
}

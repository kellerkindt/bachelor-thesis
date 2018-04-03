
mod bindings;

pub use bindings::*;


const CLIENT_TYPE_SENSOR  : ClientType_t = ClientType::ClientType_sensor  as ClientType_t;
const CLIENT_TYPE_VEHICLE : ClientType_t = ClientType::ClientType_vehicle as ClientType_t;

impl ClientType {
    pub fn from(t: ClientType_t) -> Option<ClientType> {
        match t {
            CLIENT_TYPE_SENSOR  => Some(ClientType::ClientType_sensor),
            CLIENT_TYPE_VEHICLE => Some(ClientType::ClientType_vehicle),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn client_type_t_into_client_type() {
        assert_eq!(ClientType::ClientType_sensor, ClientType::from(CLIENT_TYPE_SENSOR));
        assert_eq!(ClientType::ClientType_vehicle, ClientType::from(CLIENT_TYPE_VEHICLE));
    }
}
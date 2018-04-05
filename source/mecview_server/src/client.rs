
use async::CommandProcessor;

pub struct Client {
    variant: Variant,
}

impl Client {
    fn process_command_on_variant_unknown(&mut self, command: Command) -> Result<(), ()> {
        match command {
            Command::UpdateVariant(variant) => self.set_variant(variant),
        };
        Ok(())
    }

    fn process_command_on_variant_sensor(&mut self, command: Command) -> Result<(), ()> {
        match command {
            Command::UpdateVariant(_) => return Err(()),
        };
        Ok(())
    }

    fn process_command_on_variant_vehicle(&mut self, command: Command) -> Result<(), ()> {
        match command {
            Command::UpdateVariant(_) => return Err(()),
        };
        Ok(())
    }

    fn set_variant(&mut self, variant: Variant) {
        trace!("Client is now a {:?}", variant);
        self.variant = variant;
    }
}

impl CommandProcessor<Command> for Client {
    fn process_command(&mut self, command: Command) -> Result<(), ()> {
        match self.variant {
            Variant::Unknown => self.process_command_on_variant_unknown(command),
            Variant::Sensor  => self.process_command_on_variant_sensor(command),
            Variant::Vehicle => self.process_command_on_variant_vehicle(command),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            variant: Variant::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Unknown,
    Sensor,
    Vehicle,
}

#[derive(Debug, Clone)]
pub enum Command {
    UpdateVariant(Variant),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_variant_is_unknown() {
        assert_eq!(Variant::Unknown, Client::default().variant);
    }

    #[test]
    fn test_update_variant_cannot_change_if_not_unknown() {
        let mut client = Client::default();
        client.variant = Variant::Vehicle;
        assert_eq!(Err(()), client.process_command(Command::UpdateVariant(Variant::Vehicle)));
        assert_eq!(Err(()), client.process_command(Command::UpdateVariant(Variant::Sensor)));
    }

    #[test]
    fn test_update_variant_for_client_type_sensor() {
        test_update_variant_for_client_type(Variant::Sensor);
    }

    #[test]
    fn test_update_variant_for_client_type_vehicle() {
        test_update_variant_for_client_type(Variant::Vehicle);
    }

    fn test_update_variant_for_client_type(variant: Variant) {
        let mut client = Client::default();
        assert_eq!(Ok(()), client.process_command(Command::UpdateVariant(variant)));
        assert_eq!(variant, client.variant);
    }
}
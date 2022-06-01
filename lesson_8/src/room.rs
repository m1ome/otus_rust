use std::collections::HashMap;

use crate::devices::DeviceInfoProvider;

pub struct Room<'a> {
    pub name: String,
    pub devices: HashMap<String, &'a dyn DeviceInfoProvider>,
}

impl<'a> Room<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: &'a dyn DeviceInfoProvider) {
        self.devices.insert(device.name(), device);
    }

    pub fn devices_names(&self) -> Vec<String> {
        self.devices.values().map(|d| d.name()).collect()
    }

    pub fn report(&self) -> String {
        let reports: Vec<String> = self.devices.values().map(|d| d.info()).collect();

        reports.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn room_works() {
        let thermo = SmartThermometer::new("thermo#1");
        let socket = SmartSocket::new("socket#1");

        let mut room = Room::new("Test Room".into());
        assert_eq!(room.devices_names(), Vec::<String>::new());

        room.add_device(&thermo);
        room.add_device(&socket);

        let mut devices = room.devices_names();
        devices.sort();

        let mut expected_devices = vec!["thermo#1", "socket#1"];
        expected_devices.sort();

        assert_eq!(devices, expected_devices);

        let report_items = room
            .report()
            .as_str()
            .split("\n")
            .collect::<Vec<&str>>()
            .len();
        assert_eq!(report_items, 2);
    }
}

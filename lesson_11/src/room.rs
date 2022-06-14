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

    pub fn add_device(&mut self, device: &'a dyn DeviceInfoProvider) -> Result<(), String> {
        if self.devices.contains_key(&device.name()) {
            return Err(format!("room already have {} device", device.name()));
        }

        self.devices.insert(device.name(), device);
        Ok(())
    }

    pub fn devices_names(&self) -> Vec<String> {
        self.devices.values().map(|d| d.name()).collect()
    }

    pub fn report(&self) -> Option<String> {
        if self.devices.is_empty() {
            return None;
        }

        let reports: Vec<String> = self.devices.values().map(|d| d.info()).collect();
        let report = reports.join("\n");

        Some(report)
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

        assert!(room.add_device(&thermo).is_ok());
        assert!(room.add_device(&socket).is_ok());

        let mut devices = room.devices_names();
        devices.sort();

        let mut expected_devices = vec!["thermo#1", "socket#1"];
        expected_devices.sort();

        assert_eq!(devices, expected_devices);

        let report_items = room
            .report()
            .unwrap()
            .as_str()
            .split("\n")
            .collect::<Vec<&str>>()
            .len();
        assert_eq!(report_items, 2);
    }
}

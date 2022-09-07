use std::collections::HashMap;
use thiserror::Error;

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

    pub fn add_device(&mut self, device: &'a dyn DeviceInfoProvider) -> anyhow::Result<()> {
        if self.devices.contains_key(&device.name()) {
            return Err(RoomError::DeviceAlreadyExists(device.name()).into());
        }

        self.devices.insert(device.name(), device);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &str) -> anyhow::Result<()> {
        match self.devices.remove(name) {
            Some(_) => Ok(()),
            None => Err(RoomError::NoSuchDevice(name.into()).into()),
        }
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

#[derive(Error, Debug)]
pub enum RoomError {
    #[error("no such device {0}")]
    NoSuchDevice(String),
    #[error("device {0} already in room")]
    DeviceAlreadyExists(String),
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn room_works() {
        let thermo = SmartThermometer::new("thermo#1");
        let socket = SmartSocket::new("socket#1");
        let unused_socket = SmartSocket::new("socket#2");

        let mut room = Room::new("Test Room".into());
        assert_eq!(room.devices_names(), Vec::<String>::new());

        assert!(room.add_device(&thermo).is_ok());
        assert!(room.add_device(&socket).is_ok());
        assert!(room.add_device(&unused_socket).is_ok());

        assert!(room.remove_device("socket#2").is_ok());
        assert!(room.remove_device("unknown_device").is_err());

        let mut devices = room.devices_names();
        devices.sort();

        let mut expected_devices = vec!["thermo#1", "socket#1"];
        expected_devices.sort_unstable();

        assert_eq!(devices, expected_devices);

        let report_items = room.report().unwrap().as_str().split('\n').count();
        assert_eq!(report_items, 2);
    }
}

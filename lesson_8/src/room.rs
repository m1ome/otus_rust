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
        let thermo = SmartThermometer::new("Test".into());
        assert_eq!(thermo.temperature, 0);

        let mut room = Room::new("Test Room".into());
        assert_eq!(room.devices_names(), Vec::<String>::new());

        room.add_device(&thermo);
        assert_eq!(room.devices_names(), vec!["Test"]);

        let report = room.report();
        assert_eq!(report, "device Test showing 0 temperature");
    }
}

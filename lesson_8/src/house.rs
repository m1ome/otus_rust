use crate::room::Room;
use std::collections::HashMap;

pub struct SmartHouse<'a> {
    pub name: String,
    pub rooms: HashMap<String, &'a Room<'a>>,
}

impl<'a> SmartHouse<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: &'a Room<'a>) {
        self.rooms.insert(room.name.clone(), room);
    }

    pub fn rooms(&self) -> Vec<&String> {
        self.rooms.keys().collect()
    }

    pub fn devices(&self, name: String) -> Result<Vec<String>, &str> {
        match self.rooms.get(&name) {
            Some(room) => Ok(room.devices_names()),
            None => Err("can't find room"),
        }
    }

    pub fn create_report(&self) -> String {
        let reports: Vec<String> = self.rooms.values().map(|r| r.report()).collect();

        format!("House {} report: \n{}", self.name, reports.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn house_works() {
        let mut house = SmartHouse::new("Test House".into());
        let mut room = Room::new("room#1".into());

        assert!(house.devices("room#1".into()).is_err());
        assert_eq!(house.rooms(), Vec::<&String>::new());

        let socket = SmartSocket::new("socket#1");
        let thermo = SmartThermometer::new("thermo#1");

        room.add_device(&socket);
        room.add_device(&thermo);

        house.add_room(&room);
        assert_eq!(house.rooms(), vec!["room#1"]);

        let mut devices = house.devices("room#1".into()).unwrap();
        devices.sort();

        let mut expected_devices = vec!["socket#1", "thermo#1"];
        expected_devices.sort();

        assert_eq!(devices, expected_devices);
    }
}

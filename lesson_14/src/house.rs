use crate::room::Room;
use std::collections::HashMap;
use thiserror::Error;

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

    pub fn add_room(&mut self, room: &'a Room<'a>) -> anyhow::Result<()> {
        if self.rooms.contains_key(&room.name) {
            return Err(HouseError::RoomAlreadyExists(room.name.clone()).into());
        }

        self.rooms.insert(room.name.clone(), room);
        Ok(())
    }

    pub fn remove_room(&mut self, room_name: &str) -> anyhow::Result<()> {
        match self.rooms.remove(room_name) {
            Some(_) => Ok(()),
            None => Err(HouseError::NoSuchRoom(room_name.into()).into()),
        }
    }

    pub fn rooms(&self) -> Option<Vec<&String>> {
        let room_names: Vec<&String> = self.rooms.keys().collect();
        if room_names.is_empty() {
            return None;
        }

        Some(room_names)
    }

    pub fn devices(&self, name: String) -> anyhow::Result<Vec<String>> {
        match self.rooms.get(&name) {
            Some(room) => Ok(room.devices_names()),
            None => Err(HouseError::NoSuchRoom(name).into()),
        }
    }

    pub fn create_report(&self) -> Option<String> {
        if self.rooms.is_empty() {
            return None;
        }

        let reports: Vec<String> = self
            .rooms
            .values()
            .map(|r| match r.report() {
                None => format!("empty report for {}", r.name),
                Some(report) => report,
            })
            .collect();
        let report = format!("House {} report: \n{}", self.name, reports.join("\n"));

        Some(report)
    }
}

#[derive(Error, Debug)]
pub enum HouseError {
    #[error("can't find room {0}")]
    NoSuchRoom(String),

    #[error("room {0} not exist")]
    RoomAlreadyExists(String),
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn house_works() {
        let mut house = SmartHouse::new("Test House".into());
        let mut room = Room::new("room#1".into());
        let second_room = Room::new("room#2".into());

        assert!(house.devices("room#1".into()).is_err());
        assert_eq!(house.rooms(), None);

        let socket = SmartSocket::new("socket#1");
        let thermo = SmartThermometer::new("thermo#1");

        assert!(room.add_device(&socket).is_ok());
        assert!(room.add_device(&thermo).is_ok());

        assert!(room.add_device(&socket).is_err());

        assert!(house.add_room(&room).is_ok());
        assert!(house.add_room(&room).is_err());
        assert!(house.add_room(&second_room).is_ok());
        assert!(house.remove_room("room#2").is_ok());
        assert!(house.remove_room("room#2").is_err());
        assert_eq!(house.rooms(), Some(vec![&String::from("room#1")]));

        let mut devices = house.devices("room#1".into()).unwrap();
        devices.sort();

        let mut expected_devices = vec!["socket#1", "thermo#1"];
        expected_devices.sort_unstable();

        assert_eq!(devices, expected_devices);
    }
}

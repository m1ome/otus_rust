use crate::schema::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct House {
    pub id: i32,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = house)]
pub struct NewHouse {
    pub name: String,
}

#[derive(Queryable, Serialize, Associations, Deserialize, Debug)]
#[diesel(belongs_to(House))]
#[diesel(table_name = room)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub house_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = room)]
pub struct NewRoom {
    pub name: String,
    pub house_id: i32,
}

#[derive(Queryable, Serialize, Associations, Deserialize, Debug)]
#[diesel(belongs_to(Room))]
#[diesel(table_name = device)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub type_: String,
    pub state: serde_json::Value,
    pub room_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = device)]
pub struct NewDevice {
    pub name: String,
    pub type_: String,
    pub state: serde_json::Value,
    pub room_id: i32,
}

pub enum DeviceInfo {
    Socket(SocketInfo),
    Thermo(ThermoInfo),
}

#[derive(Serialize, Deserialize)]
pub struct SocketInfo {
    pub enabled: bool,
    pub power: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ThermoInfo {
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub house: House,
    pub rooms: Vec<Room>,
    pub devices: Vec<Device>,
}

use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::Value;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_house(conn: &mut PgConnection, house_name: String) -> Result<House, DbError> {
    let new_house = NewHouse { name: house_name };
    let res = diesel::insert_into(house::table)
        .values(&new_house)
        .get_result(conn)?;

    Ok(res)
}

pub fn get_house(conn: &mut PgConnection, house_id: i32) -> Result<House, DbError> {
    let res = house::table.find(house_id).first::<House>(conn)?;
    Ok(res)
}

pub fn delete_house(conn: &mut PgConnection, house_id: i32) -> Result<bool, DbError> {
    diesel::delete(house::table.find(house_id))
        .execute(conn)
        .map(|_| Ok(true))?
}

pub fn create_room(
    conn: &mut PgConnection,
    room_name: String,
    house_id: i32,
) -> Result<Room, DbError> {
    let new_room = NewRoom {
        name: room_name,
        house_id,
    };
    let room = diesel::insert_into(room::table)
        .values(&new_room)
        .get_result(conn)?;

    Ok(room)
}

pub fn get_room(conn: &mut PgConnection, room_id: i32) -> Result<Room, DbError> {
    let res = room::table.find(room_id).first::<Room>(conn)?;
    Ok(res)
}

pub fn delete_room(conn: &mut PgConnection, room_id: i32) -> Result<bool, DbError> {
    diesel::delete(room::table.find(room_id))
        .execute(conn)
        .map(|_| Ok(true))?
}

pub fn create_device(
    conn: &mut PgConnection,
    room_id: i32,
    device_name: String,
    device: DeviceInfo,
) -> Result<Device, DbError> {
    let new_device = match device {
        DeviceInfo::Thermo(thermo) => NewDevice {
            name: device_name,
            type_: "thermo".to_string(),
            state: Value::String(serde_json::to_string(&thermo).unwrap()),
            room_id,
        },
        DeviceInfo::Socket(socket) => NewDevice {
            name: device_name,
            type_: "socket".to_string(),
            state: Value::String(serde_json::to_string(&socket).unwrap()),
            room_id,
        },
    };

    let device = diesel::insert_into(device::table)
        .values(&new_device)
        .get_result(conn)?;

    Ok(device)
}

pub fn view_device(conn: &mut PgConnection, device_id: i32) -> Result<Device, DbError> {
    let res = device::table
        .filter(device::room_id.eq(device_id))
        .first::<Device>(conn)?;
    Ok(res)
}

pub fn delete_device(conn: &mut PgConnection, device_id: i32) -> Result<bool, DbError> {
    diesel::delete(device::table.find(device_id))
        .execute(conn)
        .map(|_| Ok(true))?
}

pub fn house_report(conn: &mut PgConnection, house_id: i32) -> Result<Report, DbError> {
    let house = house::table.find(house_id).first::<House>(conn)?;
    let rooms = room::table
        .filter(room::house_id.eq(house_id))
        .load::<Room>(conn)?;
    let devices = device::table
        .filter(device::room_id.eq_any(rooms.iter().map(|r| r.id)))
        .load::<Device>(conn)?;

    Ok(Report {
        house,
        rooms,
        devices,
    })
}

use smart_house::prelude::*;

fn main() {
    let mut socket1 = SmartSocket::new("socket#1");
    socket1.toggle();
    socket1.set_capacity(1000);
    println!("socket#1 info: {}", socket1.info());

    let mut socket2 = SmartSocket::new("socket#2");
    socket2.set_capacity(100);
    println!("socket#2 info: {}", socket2.info());

    let mut thermo = SmartThermometer::new("thermo#1");
    thermo.set_temperature(32);
    println!("thermo#1 info: {}", thermo.info());

    let mut house = SmartHouse::new("My House".into());
    let mut room1 = Room::new("room#1".into());
    room1.add_device(&socket1);
    room1.add_device(&thermo);

    let mut room2 = Room::new("room#2".into());
    room2.add_device(&socket2);

    house.add_room(&room1);
    house.add_room(&room2);

    println!("smart_house rooms: {:?}", house.rooms());
    println!(
        "smart_house devices in room#1: {:?}",
        house.devices("room#1".into()).ok()
    );
    println!(
        "smart_house devices in room#2: {:?}",
        house.devices("room#2".into()).ok()
    );
    println!("room#1 devices: {:?}", room1.devices_names());
    println!("room#2 devices: {:?}", room2.devices_names());

    let report1 = house.create_report();
    println!("report: {}", report1);
}

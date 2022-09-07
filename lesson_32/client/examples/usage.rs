use client::SmartHouseClient;
use serde_json::json;

fn main() {
    let client = SmartHouseClient::new("http://localhost:3000".to_string());
    let house = client.create_house("house".to_string()).unwrap();
    let room = client.create_room(house.id, "room".to_string()).unwrap();
    client
        .create_thermo(room.id, "thermo".to_string(), 100.0)
        .unwrap();
    client
        .create_socket(room.id, "socket".to_string(), true, 220.0)
        .unwrap();

    let report = client.house_report(house.id).unwrap();
    println!("report: {}", json!(report));

    client.delete_house(house.id).unwrap();
}

use std::collections::HashMap;

struct Room<'a> {
    name: String,
    devices: HashMap<String, &'a dyn DeviceInfoProvider>,
}

impl<'a> Room<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }

    fn add_device(&mut self, device: &'a dyn DeviceInfoProvider) {
        self.devices.insert(device.name(), device);
    }

    fn devices_names(&self) -> Vec<String> {
        self.devices.values().map(|d| d.name()).collect()
    }

    fn report(&self) -> String {
        let reports: Vec<String> = self.devices.values().map(|d| d.info()).collect();

        reports.join("\n")
    }
}

struct SmartHouse<'a> {
    name: String,
    rooms: HashMap<String, &'a Room<'a>>,
}

impl<'a> SmartHouse<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    fn add_room(&mut self, room: &'a Room<'a>) {
        self.rooms.insert(room.name.clone(), room);
    }

    fn rooms(&self) -> Vec<&String> {
        self.rooms.keys().collect()
    }

    fn devices(&self, name: String) -> Result<Vec<String>, &str> {
        match self.rooms.get(&name) {
            Some(room) => Ok(room.devices_names()),
            None => Err("can't find room"),
        }
    }

    fn create_report(&self) -> String {
        let reports: Vec<String> = self.rooms.values().map(|r| r.report()).collect();

        format!("House {} report: \n{}", self.name, reports.join("\n"))
    }
}

trait DeviceInfoProvider {
    fn name(&self) -> String;
    fn info(&self) -> String;
}

struct SmartSocket<'a> {
    name: &'a str,
    enabled: bool,
    capacity: u64,
}

impl<'a> SmartSocket<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            enabled: false,
            capacity: 0,
        }
    }

    fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    fn set_capacity(&mut self, capacity: u64) {
        self.capacity = capacity;
    }

    fn capacity(&self) -> u64 {
        self.capacity
    }
}

impl<'a> DeviceInfoProvider for SmartSocket<'a> {
    fn name(&self) -> String {
        self.name.into()
    }
    fn info(&self) -> String {
        let enabled = if self.enabled { "enabled " } else { "disabled" };
        format!(
            "device {} is {} and have capacity {}",
            self.name,
            enabled,
            self.capacity()
        )
    }
}

struct SmartThermometer<'a> {
    name: &'a str,
    temperature: i64,
}

impl<'a> SmartThermometer<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            temperature: 0,
        }
    }

    fn set_temperature(&mut self, temp: i64) {
        self.temperature = temp;
    }
}

impl<'a> DeviceInfoProvider for SmartThermometer<'a> {
    fn name(&self) -> String {
        self.name.into()
    }
    fn info(&self) -> String {
        format!(
            "device {} showing {} temperature",
            self.name, self.temperature
        )
    }
}

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

    println!("house rooms: {:?}", house.rooms());
    println!(
        "house devices in room#1: {:?}",
        house.devices("room#1".into()).ok()
    );
    println!(
        "house devices in room#2: {:?}",
        house.devices("room#2".into()).ok()
    );
    println!("room#1 devices: {:?}", room1.devices_names());
    println!("room#2 devices: {:?}", room2.devices_names());

    let report1 = house.create_report();
    println!("report: {}", report1);
}

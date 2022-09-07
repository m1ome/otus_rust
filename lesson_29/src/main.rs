#[derive(Debug, Default)]
pub struct Home {
    name: String,
    rooms: u64,
    area: u64,
    location: String,
}

impl Home {
    pub fn builder() -> HomeBuilder {
        HomeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct HomeBuilder {
    name: String,
    rooms: u64,
    area: u64,
    location: String,
}

impl HomeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn rooms(&mut self, rooms: u64) -> &mut Self {
        self.rooms = rooms;
        self
    }

    pub fn area(&mut self, area: u64) -> &mut Self {
        self.area = area;
        self
    }

    pub fn location(&mut self, location: String) -> &mut Self {
        self.location = location;
        self
    }

    pub fn build(&mut self) -> Home {
        Home {
            name: self.name.clone(),
            rooms: self.rooms,
            area: self.area,
            location: self.location.clone(),
        }
    }
}

pub struct SafeHome(Home);

impl std::fmt::Debug for SafeHome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SafeHome")
            .field("name", &self.0.name)
            .field("rooms", &self.0.rooms)
            .field("area", &self.0.area)
            .field("location", &String::from("[*******]"))
            .finish()
    }
}

fn main() {
    println!("Using builder pattern to create a house");
    let home = Home::builder()
        .name(String::from("My Home"))
        .rooms(3)
        .area(100)
        .location(String::from("My Location"))
        .build();

    println!(
        "Home {} have {} rooms with overall area {} at {}",
        home.name, home.rooms, home.area, home.location
    );

    println!("Using newtype for safe home");
    let safe_home = SafeHome(home);
    println!("home debugging info: {:?}", safe_home);
}

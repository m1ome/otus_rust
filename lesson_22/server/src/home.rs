use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Home {
    sockets: Arc<DashMap<String, Socket>>,
    thermos: Arc<DashMap<String, Thermo>>,
}

impl Home {
    pub fn socket_info(&self, socket_id: String) -> Option<String> {
        Some(self.sockets.get(&socket_id)?.info())
    }

    pub fn thermo_info(&self, thermo_id: String) -> Option<String> {
        Some(self.thermos.get(&thermo_id)?.info())
    }

    pub fn create_socket(&self, socket_id: String, power: u64, state: bool) -> Option<String> {
        let socket_entry = self.sockets.entry(socket_id.clone());
        match socket_entry {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                let socket = Socket::new(&socket_id, power, state);
                v.insert(socket);
                Some(socket_id)
            }
        }
    }

    pub fn create_thermo(&self, thermo_id: String, temp: i64) -> Option<String> {
        let thermo_entry = self.thermos.entry(thermo_id.clone());
        match thermo_entry {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                let thermo = Thermo::new(&thermo_id, temp);
                v.insert(thermo);
                Some(thermo_id)
            }
        }
    }

    pub fn toggle_socket(&self, socket_id: &str) -> Option<String> {
        let mut socket = self.sockets.get_mut(socket_id)?;
        socket.toggle();
        Some(socket_id.into())
    }

    pub fn set_thermo(&self, thermo_id: &str, temp: i64) -> Option<String> {
        let mut thermo = self.thermos.get_mut(thermo_id)?;
        thermo.set_temp(temp);
        Some(thermo_id.into())
    }
}

pub struct Socket {
    name: String,
    power: u64,
    state: bool,
}

impl Socket {
    pub fn new(name: &str, power: u64, state: bool) -> Self {
        Self {
            name: String::from(name),
            power,
            state,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Socket {} state is {}, power is {}",
            self.name, self.state, self.power
        )
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
    }
}

pub struct Thermo {
    name: String,
    temp: i64,
}

impl Thermo {
    pub fn new(name: &str, temp: i64) -> Self {
        Self {
            name: String::from(name),
            temp,
        }
    }

    pub fn info(&self) -> String {
        format!("Thermo {} temperature is {}", self.name, self.temp)
    }

    pub fn set_temp(&mut self, temp: i64) {
        self.temp = temp
    }
}

#[cfg(test)]
mod tests {
    use crate::Home;

    #[test]
    fn fetch_after_append() {
        let home = Home::default();

        let socket_id_1 = "socket_1".into();
        let socket_id_2 = "socket_2".into();

        let socket1 = home.create_socket(socket_id_1, 100, true).unwrap();
        let socket2 = home.create_socket(socket_id_2, 50, false).unwrap();

        home.toggle_socket(&socket1).unwrap();
        home.toggle_socket(&socket2).unwrap();

        let info = home.socket_info(socket1);
        println!("message: {:?}", info);
    }
}

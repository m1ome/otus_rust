use std::net::ToSocketAddrs;
use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct Client {
    stp: StpClient,
}

impl Client {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    pub fn fetch_socket(&mut self, socket_id: &str) -> RequestResult {
        let request = format!("fetch_socket|||{}", socket_id);
        self.stp.send_request(request)
    }

    pub fn create_socket(&mut self, socket_id: &str, power: &str, state: &str) -> RequestResult {
        let request = format!("create_socket|||{}|||{}|||{}", socket_id, power, state);
        self.stp.send_request(request)
    }

    pub fn toggle_socket(&mut self, socket_id: &str) -> RequestResult {
        let request = format!("toggle_socket|||{}", socket_id);
        self.stp.send_request(request)
    }

    pub fn fetch_thermo(&mut self, thermo_id: &str) -> RequestResult {
        let request = format!("fetch_thermo|||{}", thermo_id);
        self.stp.send_request(request)
    }

    pub fn create_thermo(&mut self, thermo_id: &str, temp: &str) -> RequestResult {
        let request = format!("create_thermo|||{}|||{}", thermo_id, temp);
        self.stp.send_request(request)
    }

    pub fn set_thermo(&mut self, thermo_id: &str, temp: &str) -> RequestResult {
        let request = format!("set_thermo|||{}|||{}", thermo_id, temp);
        self.stp.send_request(request)
    }    
}

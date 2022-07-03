use std::net::ToSocketAddrs;
use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct SocketClient {
    stp: StpClient,
}

impl SocketClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    pub fn fetch(&mut self, socket_id: &str) -> RequestResult {
        let request = format!("fetch|||{}", socket_id);
        self.stp.send_request(request)
    }

    pub fn create_socket(&mut self, socket_id: &str, power: &str, state: &str) -> RequestResult {
        let request = format!("create|||{}|||{}|||{}", socket_id, power, state);
        self.stp.send_request(request)
    }

    pub fn toggle_socket(&mut self, socket_id: &str) -> RequestResult {
        let request = format!("toggle|||{}", socket_id);
        self.stp.send_request(request)
    }
}

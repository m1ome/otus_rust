use serde_json::json;
use server::models::*;

pub struct SmartHouseClient {
    endpoint: String,
    client: reqwest::blocking::Client,
}

impl SmartHouseClient {
    pub fn new(endpoint: String) -> SmartHouseClient {
        let client = reqwest::blocking::Client::new();
        SmartHouseClient { endpoint, client }
    }

    pub fn create_house(&self, name: String) -> Result<House, String> {
        self.client
            .post(self.url("house"))
            .json(&NewHouse { name })
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn view_house(&self, id: i32) -> Result<House, String> {
        self.client
            .get(self.url(&format!("house/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn delete_house(&self, id: i32) -> Result<bool, String> {
        self.client
            .delete(self.url(&format!("house/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .map(|res| res.status().is_success())
    }

    pub fn create_room(&self, house_id: i32, name: String) -> Result<Room, String> {
        self.client
            .post(self.url("room"))
            .json(&NewRoom { name, house_id })
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn view_room(&self, id: i32) -> Result<Room, String> {
        self.client
            .get(self.url(&format!("room/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn delete_room(&self, id: i32) -> Result<bool, String> {
        self.client
            .delete(self.url(&format!("room/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .map(|res| res.status().is_success())
    }

    pub fn create_socket(
        &self,
        room_id: i32,
        name: String,
        enabled: bool,
        power: f32,
    ) -> Result<Device, String> {
        self.client
            .post(self.url("device"))
            .json(&NewDevice {
                name,
                room_id,
                type_: "socket".to_string(),
                state: json!(SocketInfo { enabled, power }),
            })
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn create_thermo(
        &self,
        room_id: i32,
        name: String,
        temperature: f32,
    ) -> Result<Device, String> {
        self.client
            .post(self.url("device"))
            .json(&NewDevice {
                name,
                room_id,
                type_: "thermo".to_string(),
                state: json!(ThermoInfo { temperature }),
            })
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn view_device(&self, id: i32) -> Result<Device, String> {
        self.client
            .get(self.url(&format!("device/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    pub fn delete_device(&self, id: i32) -> Result<bool, String> {
        self.client
            .delete(self.url(&format!("device/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .map(|res| res.status().is_success())
    }

    pub fn house_report(&self, id: i32) -> Result<Report, String> {
        self.client
            .get(self.url(&format!("report/{}", id)))
            .send()
            .map_err(|e| e.to_string())
            .and_then(|res| res.json().map_err(|e| e.to_string()))
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.endpoint, path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

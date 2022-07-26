use crate::home::Home;
use std::str::Split;

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    home: Home,
}

impl RequestHandler {
    pub fn new(home: Home) -> Self {
        Self { home }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next();
        match command {
            "create_socket" => self.create_socket(request),
            "fetch_socket" => self.fetch_socket(request),
            "toggle_socket" => self.toggle_socket(request),
            "create_thermo" => self.create_thermo(request),
            "fetch_thermo" => self.fetch_thermo(request),
            "set_thermo" => self.set_thermo(request),
            _ => "Bad command".into(),
        }
    }

    fn fetch_socket(&self, mut request: Request) -> String {
        let socket_id = request.next();
        if socket_id.is_empty() {
            return "Provide socket id".into();
        }

        match self.home.socket_info(socket_id.into()) {
            Some(info) => info,
            None => "Unknown socket".into(),
        }
    }

    fn fetch_thermo(&self, mut request: Request) -> String {
        let thermo_id = request.next();
        if thermo_id.is_empty() {
            return "Provide thermo id".into();
        }

        match self.home.thermo_info(thermo_id.into()) {
            Some(info) => info,
            None => "Unknown thermo".into()
        }
    }

    fn create_socket(&mut self, mut request: Request) -> String {
        let socket_id = request.next();
        if socket_id.is_empty() {
            return "Provide socket id".into();
        }

        let power = request.next();
        if power.is_empty() {
            return "Provide socket power".into();
        }

        let state = request.next();
        if state.is_empty() {
            return "Provide socket state".into();
        }

        let power_value = power.parse().unwrap_or(0);

        match self
            .home
            .create_socket(socket_id.into(), power_value, state == "true")
        {
            Some(r) => format!("Socket `{}` created", r),
            None => format!("Socket `{}` already exists", socket_id),
        }
    }

    fn create_thermo(&mut self, mut request: Request) -> String {
        let thermo_id = request.next();
        if thermo_id.is_empty() {
            return "Provide thermo id".into()
        }

        let temp = request.next();
        if temp.is_empty() {
            return "Provide thermo power".into();
        }

        let temp_value = temp.parse().unwrap_or(0);
        match self.home.create_thermo(thermo_id.into(), temp_value) {
            Some(r) => format!("Thermo `{}` created", r),
            None => format!("Thermo `{}` already exists", thermo_id)
        }
    }

    fn toggle_socket(&mut self, mut request: Request) -> String {
        let socket_id = request.next();
        if socket_id.is_empty() {
            return "Select socket id".into();
        }

        match self.home.toggle_socket(socket_id) {
            Some(_) => format!("Socket `{}` toggled", socket_id),
            None => "Bad socket".into(),
        }
    }

    fn set_thermo(&mut self, mut request: Request) -> String {
        let thermo_id = request.next();
        if thermo_id.is_empty() {
            return "Select thermo id".into()
        }

        let temp = request.next();
        if temp.is_empty() {
            return "Provide thermo power".into();
        }        

        let temp_value = temp.parse().unwrap_or(0);
        match self.home.set_thermo(thermo_id, temp_value) {
            Some(_) => format!("Thermo `{}` set temp {}", thermo_id, temp_value),
            None => "Bad thermo".into(),            
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Home, Request, RequestHandler};

    #[test]
    fn sockets() {
        let home = Home::default();
        let mut handler = RequestHandler::new(home);

        let socket_id = String::from("socket_1");
        let req_str = format!("create_socket|||{}|||{}|||{}", socket_id, 100, false);
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            format!("Socket `{}` created", socket_id)
        );

        let req_str = format!("toggle_socket|||{}", socket_id);
        let req = Request::new(&req_str);
        handler.handle(req);

        let req_str = format!("fetch_socket|||{}", socket_id);
        let req = Request::new(&req_str);
        let fetched = handler.handle(req);

        assert_eq!(fetched, "Socket socket_1 state is true, power is 100");
    }

    #[test] 
    fn thermos() {
        let home = Home::default();
        let mut handler = RequestHandler::new(home);

        let thermo_id = String::from("thermo_1");
        let req_str = format!("create_thermo|||{}|||{}", thermo_id, 100);
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            format!("Thermo `{}` created", thermo_id)
        );

        let req_str = format!("set_thermo|||{}|||{}", thermo_id, 50);
        let req = Request::new(&req_str);
        handler.handle(req);

        let req_str = format!("fetch_thermo|||{}", thermo_id);
        let req = Request::new(&req_str);
        let fetched = handler.handle(req);

        assert_eq!(fetched, "Thermo thermo_1 temperature is 50");        
    }
}

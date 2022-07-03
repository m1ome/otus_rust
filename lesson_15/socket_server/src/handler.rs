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
            "create" => self.create_socket(request),
            "fetch" => self.fetch_socket(request),
            "toggle" => self.toggle_socket(request),
            _ => "Bad command".into(),
        }
    }

    fn fetch_socket(&self, mut request: Request) -> String {
        let socket_id = request.next();
        if socket_id.is_empty() {
            return "Provide socket id".into();
        }

        match self.home.info(socket_id.into()) {
            Some(info) => info,
            None => "Unknown socket".into(),
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

    fn toggle_socket(&mut self, mut request: Request) -> String {
        let socket_id = request.next();
        if socket_id.is_empty() {
            return "Select room id".into();
        }

        match self.home.toggle(socket_id) {
            Some(_) => format!("Socket `{}` toggled", socket_id),
            None => "Bad socket".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Home, Request, RequestHandler};

    #[test]
    fn append_fetch() {
        let home = Home::default();
        let mut handler = RequestHandler::new(home);

        let socket_id = String::from("socket_1");
        let req_str = format!("create|||{}|||{}|||{}", socket_id, 100, false);
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            format!("Socket `{}` created", socket_id)
        );

        let req_str = format!("toggle|||{}", socket_id);
        let req = Request::new(&req_str);
        handler.handle(req);

        let req_str = format!("fetch|||{}", socket_id);
        let req = Request::new(&req_str);
        let fetched = handler.handle(req);

        assert_eq!(fetched, "Socket socket_1 state is true, power is 100");
    }
}

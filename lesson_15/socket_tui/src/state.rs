use socket_client::SocketClient;
use std::io;

pub trait State {
    fn update(&mut self, client: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

impl State for Main {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Create socket
    2) Show socket
    3) Toggle socket
    Other) Exit"
        );
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => Ok(Box::new(CreateSocket)),
            "2" => Ok(Box::new(ShowSocket)),
            "3" => Ok(Box::new(ToggleSocket)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct Exit;

impl State for Exit {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}

struct CreateSocket;

impl State for CreateSocket {
    fn update(&mut self, home: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter socket name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let name = buf.trim();

        println!("Etner socket power:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let power = buf.trim();

        println!("Etner socket state [true/false]:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let state = buf.trim();

        let create_result = home.create_socket(name, power, state)?;

        println!("Create socket: {}", create_result);
        Ok(Box::new(Main))
    }
}

struct ShowSocket;

impl State for ShowSocket {
    fn update(&mut self, home: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter socket name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        let info_result = home.fetch(name)?;

        println!("Socket: {}", info_result);

        Ok(Box::new(Main))
    }
}

struct ToggleSocket;

impl State for ToggleSocket {
    fn update(&mut self, home: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter socket name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        println!("Result: {}", home.toggle_socket(name)?);

        Ok(Box::new(Main))
    }
}

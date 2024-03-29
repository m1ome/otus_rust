use async_trait::async_trait;
use client::Client;
use std::io;

#[async_trait]

pub trait State {
    async fn update(&mut self, client: &mut Client) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

#[async_trait]
impl State for Main {
    async fn update(&mut self, _: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Create socket
    2) Show socket
    3) Toggle socket
    4) Create thermo
    5) Show thermo
    6) Set thermo
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
            "4" => Ok(Box::new(CreateThermo)),
            "5" => Ok(Box::new(ShowThermo)),
            "6" => Ok(Box::new(SetThermo)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct Exit;

#[async_trait]
impl State for Exit {
    async fn update(&mut self, _: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}

struct CreateSocket;

#[async_trait]
impl State for CreateSocket {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
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

        let create_result = home.create_socket(name, power, state).await?;

        println!("Create socket: {}", create_result);
        Ok(Box::new(Main))
    }
}

struct ShowSocket;

#[async_trait]
impl State for ShowSocket {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter socket name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        let info_result = home.fetch_socket(name).await?;

        println!("Socket: {}", info_result);

        Ok(Box::new(Main))
    }
}

struct ToggleSocket;

#[async_trait]
impl State for ToggleSocket {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter socket name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        println!("Result: {}", home.toggle_socket(name).await?);

        Ok(Box::new(Main))
    }
}

struct CreateThermo;

#[async_trait]
impl State for CreateThermo {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter thermo name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let name = buf.trim();

        println!("Etner thermo temperature:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let temp = buf.trim();

        let create_result = home.create_thermo(name, temp).await?;

        println!("Create thermo: {}", create_result);
        Ok(Box::new(Main))
    }
}

struct ShowThermo;

#[async_trait]
impl State for ShowThermo {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter thermo name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let name = buf.trim();
        let info_result = home.fetch_thermo(name).await?;

        println!("Thermo: {}", info_result);

        Ok(Box::new(Main))
    }
}

struct SetThermo;

#[async_trait]
impl State for SetThermo {
    async fn update(&mut self, home: &mut Client) -> Result<Box<dyn State>, anyhow::Error> {
        println!("Enter thermo name:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let name = buf.trim();

        println!("Etner thermo temperature:");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let temp = buf.trim();

        println!("Result: {}", home.set_thermo(name, temp).await?);

        Ok(Box::new(Main))
    }
}

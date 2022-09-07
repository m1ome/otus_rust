use iced::{
    button, text_input, Alignment, Button, Column, Element, Sandbox, Settings, Text, TextInput,
};
use std::fs;
use tokio::runtime::Runtime;

fn main() {
    Socket::run(
        Settings{
            window: iced::window::Settings{
                size: (400, 300),
                ..Default::default()
            },
            ..Settings::default()
        }
    ).expect("Failed to run GUI");
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}

pub struct BlockingClient {
    inner: client::Client,
    rt: Runtime,
}

impl BlockingClient {
    pub fn new(addr: String) -> Self {
        let rt = Runtime::new().unwrap();
        let inner = rt.block_on(client::Client::new(addr)).unwrap();

        BlockingClient { inner, rt }
    }

    pub fn create_socket(&mut self, socket_id: &str, power: &str, state: &str) -> String {
        self.rt
            .block_on(self.inner.create_socket(socket_id, power, state))
            .unwrap()
    }

    pub fn toggle_socket(&mut self, socket_id: &str) -> String {
        self.rt
            .block_on(self.inner.toggle_socket(socket_id))
            .unwrap()
    }
    
    pub fn fetch_socket(&mut self, socket_id: &str) -> String {
        self.rt
            .block_on(self.inner.fetch_socket(socket_id))
            .unwrap()
    }
}

struct Socket {
    id: String,
    power: f64,
    state: bool,
    created: bool,
    client: BlockingClient,

    create_state: text_input::State,
    button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSocket,
    CreateSocket,
    ChangePower(String),
}

fn socket_create_dashboard(socket: &mut Socket) -> Element<Message> {
    Column::new()
    .push(Text::new("Create New Socket").size(50))
    .push(TextInput::new(
        &mut socket.create_state,
        "Enter socket name",
        &socket.power.to_string(),
        Message::ChangePower,
    ))
    .push(
        Button::new(&mut socket.button_state, Text::new("Create"))
            .on_press(Message::CreateSocket),
    )
    .padding(20)
    .align_items(Alignment::Center)
    .into()
}  

fn socket_dashboard(socket: &mut Socket) -> Element<Message> {
    Column::new()
    .push(Text::new("Socket Dashboard").size(50))
    .push(Text::new(format!("Socket ID: {}", socket.id)))
    .push(Text::new(format!("Power: {}", socket.power)))
    .push(Text::new(format!("State: {}", socket.state)))
    .push(
        Button::new(&mut socket.button_state, Text::new("Toggle"))
            .on_press(Message::ToggleSocket),
    )
    .padding(20)
    .align_items(Alignment::Center)
    .into()
}

impl Sandbox for Socket {
    type Message = Message;

    fn new() -> Self {
        let addr = get_server_addr();
        let client = BlockingClient::new(addr);

        Self {
            id: "socket#1".to_string(),
            power: 0.0,
            state: false,
            created: false,
            client,

            create_state: text_input::State::new(),
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        "Socket".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleSocket => {
                self.state = !self.state;
                self.client.toggle_socket(&self.id);
            }
            Message::CreateSocket => {
                self.created = true;
                self.client.create_socket(
                    &self.id,
                    &self.power.to_string(),
                    &self.state.to_string(),
                );
            }
            Message::ChangePower(power) => {
                self.power = power.parse().unwrap_or(0.0);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.created {
            true => socket_dashboard(self),
            false => {
                let socket = self.client.fetch_socket(&self.id);
                match socket.as_str() {
                    "Unknown socket" => {
                        socket_create_dashboard(self)
                    },
                    _ => {
                        let socket_data: Vec<&str> = socket.split(',').collect();
                        self.created = true;
                        self.power = socket_data[2].parse().unwrap();
                        self.state = socket_data[1].parse().unwrap();

                        socket_dashboard(self)
                    }
                }
            },
        }
    }  
}

use client::Client;
use state::{Main, State};
use std::fs;

mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = get_server_addr();
    let mut client = Client::new(addr).await?;

    let mut state: Box<dyn State> = Box::new(Main);
    while !state.exit() {
        state = state.update(&mut client).await?;
    }

    Ok(())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}

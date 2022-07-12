use rand::Rng;
use std::env;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let port = env::var("PORT").unwrap_or_else(|_| "127.0.0.1:34255".to_string());
    let server = env::var("SERVER").unwrap_or_else(|_| "127.0.0.1:34254".to_string());

    println!("connecting to server {server}");
    let socket = UdpSocket::bind(port).expect("couldn't bind to address");

    loop {
        let temp: i64 = rng.gen_range(-10..35);
        socket
            .send_to(&temp.to_ne_bytes(), &server)
            .expect("couldn't send data");
        println!("sent {temp} to a thermo");
        thread::sleep(Duration::from_secs(1));
    }
}

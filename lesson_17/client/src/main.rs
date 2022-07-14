use rand::Rng;
use std::env;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn write_bytes_to_socket(buf: &[u8; 8], socket: &UdpSocket, server: &String) -> Result<(), String> {
    let mut bytes_written = 0;

    while bytes_written < buf.len() {
        let data_left = buf.get(bytes_written..).unwrap();
        let n = socket
            .send_to(data_left, &server)
            .map_err(|e| e.to_string())?;
        bytes_written += n;
    }

    Ok(())
}

fn main() {
    let mut rng = rand::thread_rng();
    let port = env::var("PORT").unwrap_or_else(|_| "127.0.0.1:34255".to_string());
    let server = env::var("SERVER").unwrap_or_else(|_| "127.0.0.1:34254".to_string());

    println!("connecting to server {server}");
    let socket = UdpSocket::bind(port).expect("couldn't bind to address");

    loop {
        let temp: i64 = rng.gen_range(-10..35);
        write_bytes_to_socket(&temp.to_ne_bytes(), &socket, &server).unwrap();

        println!("sent {temp} to a thermo");
        thread::sleep(Duration::from_secs(1));
    }
}

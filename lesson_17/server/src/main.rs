use std::env;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Thermo {
    temp: i64,
}

impl Thermo {
    pub fn new(temp: i64) -> Self {
        Self { temp }
    }

    pub fn set_temp(&mut self, temp: i64) {
        self.temp = temp;
    }
}

fn read_bytes_from_socket(buf: &mut [u8; 8], socket: &UdpSocket) -> Result<(), String> {
    let mut bytes_read = 0;

    while bytes_read < buf.len() {
        let data_left = buf.get_mut(bytes_read..).unwrap();
        let (n, _src) = socket.recv_from(data_left).map_err(|e| e.to_string())?;

        bytes_read += n;
    }

    Ok(())
}

fn main() {
    let thermo = Arc::new(RwLock::new(Thermo::new(0)));
    let thermo_ref = thermo.clone();
    let port = env::var("PORT").unwrap_or_else(|_| "127.0.0.1:34254".to_string());

    println!("starting server on port {port}");
    let socket = UdpSocket::bind(port).unwrap();
    let (tx, rx) = mpsc::channel();

    let thread_receive = thread::spawn(move || {
        let mut buf = [0; 8];
        loop {
            read_bytes_from_socket(&mut buf, &socket).unwrap();
            let thermo = i64::from_ne_bytes(buf);
            tx.send(thermo).unwrap();
        }
    });

    let thread_work = thread::spawn(move || loop {
        let stuff = rx.recv().unwrap();
        let mut t = thermo.write().unwrap();
        t.set_temp(stuff);
    });

    let thread_info = thread::spawn(move || loop {
        let temp = thermo_ref.read().unwrap().temp;
        println!("current thermo temp {temp}");
        thread::sleep(Duration::from_secs(1));
    });

    thread_receive.join().unwrap();
    thread_work.join().unwrap();
    thread_info.join().unwrap();
}

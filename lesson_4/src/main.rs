struct Socket {}

impl Socket {
    fn new(_name: String) -> Self {
        todo!()
    }

    fn toggle(&mut self) {
        todo!()
    }

    fn capacity(&self) -> u64 {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}

struct Thermo {}

impl Thermo {
    fn new() -> Self {
        todo!()
    }

    fn temperature(&self) -> i64 {
        todo!()
    }
}

fn main() {
    let mut socket = Socket::new(String::from("My New Socket"));
    socket.toggle();
    println!("socket name: {}", socket.name());
    println!("socket capacity: {}", socket.capacity());

    let thermo = Thermo::new();
    println!("thermo value: {}", thermo.temperature());
}

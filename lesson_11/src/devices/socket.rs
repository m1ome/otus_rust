use crate::devices::DeviceInfoProvider;

pub struct SmartSocket<'a> {
    pub name: &'a str,
    pub enabled: bool,
    pub capacity: u64,
}

impl<'a> SmartSocket<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            enabled: false,
            capacity: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_capacity(&mut self, capacity: u64) {
        self.capacity = capacity;
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}

impl<'a> DeviceInfoProvider for SmartSocket<'a> {
    fn name(&self) -> String {
        self.name.into()
    }
    fn info(&self) -> String {
        let enabled = if self.enabled { "enabled" } else { "disabled" };
        format!(
            "device {} is {} and have capacity {}",
            self.name,
            enabled,
            self.capacity()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::socket::SmartSocket;
    use crate::devices::DeviceInfoProvider;

    #[test]
    fn socket_works() {
        let mut socket = SmartSocket::new("Test");
        assert_eq!(socket.enabled, false);
        assert_eq!(socket.capacity, 0);

        socket.set_capacity(100);
        socket.toggle();
        assert_eq!(socket.capacity, 100);
        assert_eq!(socket.enabled, true);

        let info = socket.info();
        assert_eq!(info, "device Test is enabled and have capacity 100");
    }
}

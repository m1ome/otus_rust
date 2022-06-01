use crate::devices::DeviceInfoProvider;

pub struct SmartThermometer<'a> {
    pub name: &'a str,
    pub temperature: i64,
}

impl<'a> SmartThermometer<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            temperature: 0,
        }
    }

    pub fn set_temperature(&mut self, temp: i64) {
        self.temperature = temp;
    }
}

impl<'a> DeviceInfoProvider for SmartThermometer<'a> {
    fn name(&self) -> String {
        self.name.into()
    }
    fn info(&self) -> String {
        format!(
            "device {} showing {} temperature",
            self.name, self.temperature
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::thermo::SmartThermometer;
    use crate::devices::DeviceInfoProvider;

    #[test]
    fn thermo_works() {
        let mut thermo = SmartThermometer::new("Test".into());
        assert_eq!(thermo.temperature, 0);

        thermo.set_temperature(100);
        assert_eq!(thermo.temperature, 100);
        assert_eq!(thermo.name, "Test");

        let info = thermo.info();
        assert_eq!(info, "device Test showing 100 temperature");
    }
}

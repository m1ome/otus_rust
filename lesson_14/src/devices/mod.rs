pub mod socket;
pub mod thermo;

pub mod prelude {
    pub use crate::devices::socket::SmartSocket;
    pub use crate::devices::thermo::SmartThermometer;
    pub use crate::devices::DeviceInfoProvider;
}

pub trait DeviceInfoProvider {
    fn name(&self) -> String;
    fn info(&self) -> String;
}

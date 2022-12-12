use crate::device::{Device, Keyboard};

use super::{Driver, Error};

/// Evdev is a wrapper around the evdev library.
pub struct Evdev {}

impl Evdev {
    pub fn new() -> Self {
        Self {}
    }
}
impl Driver for Evdev {
    /// Crawls `/dev/input` for evdev devices.
    /// If name of device can't be read, will use "N/A".
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, Error> {
        Ok(evdev::enumerate()
            .filter(|(_, d)| d.physical_path().is_some())
            .map(|(_, d)| -> Box<dyn Device> {
                let name = d.name().unwrap_or("N/A");
                let path = d.physical_path().unwrap();
                Box::new(Keyboard::new(name, path))
            })
            .collect())
    }

    /// Print devices' name and path if found.
    fn print_devices(&self) {
        match self.get_printable_devices() {
            Ok(devices) => {
                for (name, path) in devices {
                    println!("Device {} at {}", name, path);
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }
}

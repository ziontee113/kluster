use crate::device::{Device, Keyboard};

use super::{Driver, Error};

pub struct Evdev {}

impl Evdev {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for Evdev {
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
}

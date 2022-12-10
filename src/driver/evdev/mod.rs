use crate::device::{Device, Keyboard};

use super::Driver;

pub struct Evdev {}

impl Evdev {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for Evdev {
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, super::Error> {
        let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
        let mut vec: Vec<Box<dyn Device>> = vec![];

        for d in devices {
            if let Some(path) = d.physical_path() {
                let name = d.name().unwrap_or("N/a");
                vec.push(Box::new(Keyboard::new(name, path)));
            }
        }

        Ok(vec)
    }
}

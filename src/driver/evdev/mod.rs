use super::Driver;

pub struct Evdev {}

impl Evdev {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for Evdev {
    fn get_devices(&self) -> Result<Vec<Box<dyn crate::device::Device>>, super::Error> {
        // evdev::enumerate()
        //     .filter_map(|(_, device)| {
        //         if let Some(path) = device.physical_path() {
        //             let name = device.name().unwrap_or("N/A");
        //             return Some(Keyboard::new(path) {
        //                 // name: name.to_string(),
        //                 // path: path.to_string(),
        //             });
        //         }
        //         None
        //     })
        //     .collect()
        todo!()
    }
}

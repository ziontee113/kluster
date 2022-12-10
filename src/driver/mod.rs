mod evdev;

use crate::device::Device;

enum Error {
    Unknown,
}

trait Driver {
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, Error>;
}

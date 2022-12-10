mod evdev;

use crate::device::Device;

enum Error {
    Unknown,
}

trait Driver {
    /// Returns a list of available devices.
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, Error>;
}

mod evdev;

use ::evdev::Key;

use crate::device::{Device, Keyboard};

enum Error {
    Unknown,
}

trait Driver {
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, Error>;
}

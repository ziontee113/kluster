mod evdev;

use crate::device::Device;
#[cfg(test)]
use crate::device::Keyboard;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Unknown,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Driver {
    fn get_devices(&self) -> Result<Vec<Box<dyn Device>>, Error>;
}

pub fn get_devices<T: Driver>(driver: &T) -> Result<Vec<Box<dyn Device>>, Error> {
    driver.get_devices()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_devices_returns_an_error() {
        let mut mock = MockDriver::new();
        mock.expect_get_devices()
            .times(1)
            .returning(|| Err(Error::Unknown));

        let got = get_devices(&mock);
        assert!(&got.is_err());
    }

    #[test]
    fn get_devices_returns_devices() {
        let d1 = Keyboard::new("device1", "path1");
        let d2 = Keyboard::new("device2", "path2");
        let (device1, device2) = (d1.clone(), d2.clone());
        let mut mock = MockDriver::new();
        mock.expect_get_devices()
            .times(1)
            .returning(move || Ok(vec![Box::new(d1.clone()), Box::new(d2.clone())]));

        let got = get_devices(&mock).unwrap();
        assert_eq!(2, got.len());
        assert_eq!(got.get(0).unwrap().path(), device1.path());
        assert_eq!(got.get(1).unwrap().path(), device2.path());
    }
}

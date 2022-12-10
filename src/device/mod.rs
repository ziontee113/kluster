#![allow(dead_code)]
use std::time::SystemTime;

/// A Device should be able to describe its file path in the Linux system.
pub trait Device {
    fn path(&self) -> &str;
}

/// Describes a physical keyboard device.
pub struct Keyboard {
    path: String,
    name: String,
}

impl Keyboard {
    pub fn new<T: Into<String>>(name: T, path: T) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}

impl Device for Keyboard {
    fn path(&self) -> &str {
        self.path.as_ref()
    }
}

/// Identifies a Key in a certain Device.
pub struct Key<T: Device> {
    code: u16,
    device: T,
}

impl<T: Device> Key<T> {
    pub fn new(code: u16, device: T) -> Self {
        Self { code, device }
    }
}

/// The status of a key at a certain time.
pub enum Status {
    Up,
    Down,
    Hold,
}

/// Describes an input event and its status that occurs at a certain point in time.
pub struct InputFragment<T: Device> {
    key: Key<T>,
    status: Status,
    timestamp: SystemTime,
}

impl<T: Device> InputFragment<T> {
    pub fn new(key: Key<T>, status: Status, timestamp: SystemTime) -> Self {
        Self {
            key,
            status,
            timestamp,
        }
    }
}

/// A Collection of `InputFragments`.
pub struct FragmentBundle<'a, T: 'a + Device> {
    fragments: Vec<&'a InputFragment<T>>,
}

impl<'a, T: Device> FragmentBundle<'a, T> {
    pub fn new(fragments: Vec<&'a InputFragment<T>>) -> Self {
        Self { fragments }
    }
}

impl<'a, T: Device> From<Vec<&'a InputFragment<T>>> for FragmentBundle<'a, T> {
    fn from(fragments: Vec<&'a InputFragment<T>>) -> Self {
        Self { fragments }
    }
}

#![allow(dead_code)]
use std::time::SystemTime;

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

/// Describes a physical keyboard device.
pub struct Keyboard {
    path: String,
}

impl Keyboard {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

/// A Device should be able to describe what is the file path in the Linux system.
pub trait Device {
    fn path(&self) -> &str;
}

impl Device for Keyboard {
    fn path(&self) -> &str {
        self.path.as_ref()
    }
}

/// The status of a key at a certain time.
pub enum Status {
    Up,
    Down,
    Hold,
}

/// Describes a Key event and its status that occurs in a certain time.
pub struct InputFragment<T: Device> {
    key: Key<T>,
    status: Status,
    timestamp: SystemTime,
}

impl<T: Device> InputFragment<T> {
    pub fn new(key: Key<T>, status: Status, timestamp: SystemTime) -> Self {
        Self { key, status, timestamp }
    }
}

pub struct FragmentBundle<T: Device> {
    fragments: Vec<InputFragment<T>>,
}

impl<T: Device> FragmentBundle<T> {
    pub fn new(fragments: Vec<InputFragment<T>>) -> Self {
        Self { fragments }
    }
}

impl<T: Device> From<Vec<InputFragment<T>>> for FragmentBundle<T> {
    fn from(fragments: Vec<InputFragment<T>>) -> Self {
        Self { fragments }
    }
}

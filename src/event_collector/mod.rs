use std::time::SystemTime;

pub struct Key {
    code: u16,
    path: String,
}

impl Key {
    pub fn new(code: u16, path: String) -> Self {
        Self { code, path }
    }
}

pub struct KeyboardEvent {
    key: Key,
    state: i32,
    timestamp: SystemTime,
}

impl KeyboardEvent {
    pub fn new(key: Key, state: i32, timestamp: SystemTime) -> Self {
        Self {
            key,
            state,
            timestamp,
        }
    }
}

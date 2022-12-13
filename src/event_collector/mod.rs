#[cfg(test)]
mod test;

use std::time::SystemTime;

pub enum EventKind {
    Keyboard,
    Mouse,
}

pub trait Event {
    fn event_kind(&self) -> EventKind;
}

#[derive(Debug, Clone)]
pub struct Key {
    code: u16,
    path: String,
}

impl Key {
    pub fn new(code: u16, path: &str) -> Self {
        Self {
            code,
            path: path.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyState {
    Down,
    Up,
    Hold,
    Uninitiated,
}

impl From<i32> for KeyState {
    fn from(value: i32) -> Self {
        match value {
            0 => KeyState::Up,
            1 => KeyState::Down,
            2 => KeyState::Hold,
            -1 => KeyState::Uninitiated,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    key: Key,
    state: KeyState,
    timestamp: SystemTime,
}

impl KeyboardEvent {
    pub fn new(key: Key, state: i32, timestamp: SystemTime) -> Self {
        Self {
            key,
            state: state.into(),
            timestamp,
        }
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn state(&self) -> &KeyState {
        &self.state
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Event for KeyboardEvent {
    fn event_kind(&self) -> EventKind {
        EventKind::Keyboard
    }
}

pub struct Collector {
    pending_cluster_events: Vec<KeyboardEvent>,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            pending_cluster_events: vec![],
        }
    }
    pub fn receive(&mut self, event: &KeyboardEvent) {
        let cluster_interval_limit = 20;

        if event.state == KeyState::Down {
            if self.pending_cluster_events.is_empty() {
                self.pending_cluster_events.push(event.clone());
            } else {
                let first_time = self.pending_cluster_events.first().unwrap().timestamp();
                if event
                    .timestamp()
                    .duration_since(first_time)
                    .unwrap()
                    .as_millis()
                    <= cluster_interval_limit
                {
                    self.pending_cluster_events.push(event.clone());
                } else {
                    // TODO:
                }
            }
        }
    }
}

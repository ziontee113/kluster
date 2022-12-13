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

#[derive(Debug, Clone)]
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

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn state(&self) -> i32 {
        self.state
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
        if self.pending_cluster_events.is_empty() {
            self.pending_cluster_events.push(event.clone());
        } else {
            let first_time = self.pending_cluster_events.first().unwrap().timestamp();
            if event
                .timestamp()
                .duration_since(first_time)
                .unwrap()
                .as_millis()
                <= 20
            {
                self.pending_cluster_events.push(event.clone());
            } else {
                // TODO:
            }
        }
    }
}

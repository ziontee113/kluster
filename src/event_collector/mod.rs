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

pub struct Union {
    members: Vec<KeyboardEvent>,
}

impl Union {
    pub fn new(members: Vec<KeyboardEvent>) -> Self {
        Self { members }
    }
}

pub enum InputElement {
    Key(KeyboardEvent),
    Union(Union),
}

#[derive(Default)]
pub struct Collector {
    pending_cluster: Vec<KeyboardEvent>,
    sequence: Vec<InputElement>,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn pending_cluster(&self) -> &[KeyboardEvent] {
        self.pending_cluster.as_ref()
    }

    pub fn sequence(&self) -> &[InputElement] {
        self.sequence.as_ref()
    }
}

impl Collector {
    pub fn receive(&mut self, event: &KeyboardEvent) {
        let cluster_interval_limit = 20;

        if event.state == KeyState::Down {
            if self.pending_cluster.is_empty() {
                self.pending_cluster.push(event.clone());
            } else {
                let first_time = self.pending_cluster.first().unwrap().timestamp();
                if event
                    .timestamp()
                    .duration_since(first_time)
                    .unwrap()
                    .as_millis()
                    <= cluster_interval_limit
                {
                    self.pending_cluster.push(event.clone());
                } else {
                    if !self.pending_cluster.is_empty() {
                        if self.pending_cluster.len() == 1 {
                            self.sequence
                                .push(InputElement::Key(self.pending_cluster.pop().unwrap()));
                        } else {
                            let union = InputElement::Union(Union::new(
                                self.pending_cluster.drain(0..).collect(),
                            ));

                            self.sequence.push(union);
                        }
                    }

                    self.sequence.push(InputElement::Key(event.clone()));
                }
            }
        }
    }
}

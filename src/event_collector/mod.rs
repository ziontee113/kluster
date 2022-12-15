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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    path: String,
    code: u16,
}

impl Key {
    pub fn new(path: &str, code: u16) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cluster {
    members: Vec<KeyboardEvent>,
}

impl Cluster {
    pub fn new(members: Vec<KeyboardEvent>) -> Self {
        Self { members }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputElement {
    Key(KeyboardEvent),
    Cluster(Cluster),
}

#[derive(Default)]
pub struct Collector {
    pending_cluster: Vec<KeyboardEvent>,
    sequence: Vec<InputElement>,

    current_prefix: Vec<InputElement>,
    latest_event: Option<KeyboardEvent>,
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

    pub fn latest_event(&self) -> Option<&KeyboardEvent> {
        self.latest_event.as_ref()
    }

    pub fn current_prefix(&self) -> &[InputElement] {
        self.current_prefix.as_ref()
    }
}

impl Collector {
    pub fn receive(&mut self, event: &KeyboardEvent) {
        let cluster_interval_limit = 20;

        self.latest_event = Some(event.clone());
        self.update_current_prefix();

        if event.state == KeyState::Down {
            if self.pending_cluster.is_empty() {
                self.add_to_pending_cluster(event);
            } else {
                let first_timestamp = self.pending_cluster.first().unwrap().timestamp();
                let event_within_interval_limit = event
                    .timestamp()
                    .duration_since(first_timestamp)
                    .unwrap()
                    .as_millis()
                    <= cluster_interval_limit;

                if event_within_interval_limit {
                    self.add_to_pending_cluster(event);
                }

                if !event_within_interval_limit {
                    if self.pending_cluster.len() == 1 {
                        self.transfer_pending_cluster();
                    }

                    if self.pending_cluster.len() > 1 {
                        self.form_pending_cluster();
                    }

                    // update the current_prefix if incoming event outside of interval limit
                    // do this before pushing latest `InputElement::Key` to `sequence`.
                    self.update_current_prefix();

                    self.sequence.push(InputElement::Key(event.clone()));
                }
            }
        }
    }

    /// Push a *cloned* `event` into `pending_cluster`.
    fn add_to_pending_cluster(&mut self, event: &KeyboardEvent) {
        self.pending_cluster.push(event.clone());
    }

    /// Drain all `pending_cluster` elements, create new `InputElement`s from them,
    /// then push them to `sequence`.
    fn transfer_pending_cluster(&mut self) {
        let members = self.pending_cluster.drain(..);
        for member in members {
            self.sequence.push(InputElement::Key(member));
        }
    }

    /// Form new `Cluster` from `pending_cluster`
    /// then push it to the `sequence`.
    fn form_pending_cluster(&mut self) {
        let cluster = Cluster::new(self.pending_cluster.drain(..).collect());
        let cluster = InputElement::Cluster(cluster);
        self.sequence.push(cluster);
    }

    fn update_current_prefix(&mut self) {
        self.current_prefix = self.sequence.clone();
    }
}

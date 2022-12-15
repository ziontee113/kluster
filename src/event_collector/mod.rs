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

    pub fn state(&self) -> KeyState {
        self.state.clone()
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

// ---------------------------------------------------------------------------

enum PendingClusterState {
    Pending,
    Formed(InputElement),
    Rejected(Vec<InputElement>),
}

pub struct PendingCluster {
    members: Vec<KeyboardEvent>,
    state: PendingClusterState,
}

impl Default for PendingCluster {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            state: PendingClusterState::Pending,
        }
    }
}

impl PendingCluster {
    /// Push a *cloned* `event` to `self.members`
    fn add(&mut self, event: &KeyboardEvent) {
        self.members.push(event.clone());
        self.state = PendingClusterState::Pending;
    }

    fn form(&mut self) {
        let cluster = Cluster::new(self.members.drain(..).collect());
        let element = InputElement::Cluster(cluster);
        self.state = PendingClusterState::Formed(element);
    }

    fn reject(&mut self) {
        let elements: Vec<InputElement> = self.members.drain(..).map(InputElement::Key).collect();
        self.state = PendingClusterState::Rejected(elements);
    }

    fn update(&mut self, event: &KeyboardEvent, limit: u128) {
        if event.state() == KeyState::Down {
            if self.members.is_empty() || self.incoming_event_fits_in_interval_limit(event, limit) {
                self.add(event);
            } else {
                if self.has_single_member() {
                    self.reject();
                }
                if self.has_multiple_members() {
                    self.form();
                }
            }
        }
    }

    fn incoming_event_fits_in_interval_limit(
        &mut self,
        event: &KeyboardEvent,
        cluster_interval_limit: u128,
    ) -> bool {
        let first_member_timestamp = self.members.first().unwrap().timestamp();
        event
            .timestamp()
            .duration_since(first_member_timestamp)
            .unwrap()
            .as_millis()
            > cluster_interval_limit
    }

    fn has_multiple_members(&mut self) -> bool {
        self.members.len() > 1
    }

    fn has_single_member(&mut self) -> bool {
        self.members.len() == 1
    }
}

#[derive(Default)]
pub struct Sequence {
    elements: Vec<InputElement>,
}

#[derive(Default)]
pub struct Collector {
    pending_cluster: PendingCluster,
    sequence: Sequence,
}

impl Collector {
    pub fn pending_cluster(&self) -> &PendingCluster {
        &self.pending_cluster
    }

    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
}

impl Collector {
    pub fn receive(&mut self, event: &KeyboardEvent) {
        let cluster_interval_limit = 20;

        self.pending_cluster.update(event, cluster_interval_limit);
    }
}

use crate::test_utilities::mipoch;

use super::*;

#[test]
fn receive_events_within_interval() {
    let mut collector = Collector::new();
    let event_1 = KeyboardEvent::new(Key::new(32, "path/to/kb/1"), 1, mipoch(0));
    let event_2 = KeyboardEvent::new(Key::new(33, "path/to/kb/1"), 1, mipoch(4));
    collector.receive(&event_1);
    collector.receive(&event_2);
    assert_eq!(collector.pending_cluster_events.len(), 2);
}

#[test]
fn receive_events_outside_interval() {
    let mut collector = Collector::new();
    let event_1 = KeyboardEvent::new(Key::new(32, "path/to/kb/1"), 1, mipoch(0));
    let event_2 = KeyboardEvent::new(Key::new(33, "path/to/kb/1"), 1, mipoch(22));
    collector.receive(&event_1);
    collector.receive(&event_2);
    assert_eq!(collector.pending_cluster_events.len(), 1);
}

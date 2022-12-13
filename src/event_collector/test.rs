use crate::test_utilities::mipoch;

use super::*;

#[test]
fn receive_event_within_interval() {
    let mut collector = Collector::new();

    let event_1 = KeyboardEvent::new(Key::new(32, "path/to/kb/1"), 1, mipoch(0));
    collector.receive(&event_1);
    assert_eq!(collector.pending_cluster().len(), 1);

    let event_2 = KeyboardEvent::new(Key::new(33, "path/to/kb/1"), 1, mipoch(4));
    collector.receive(&event_2);
    assert_eq!(collector.pending_cluster().len(), 2);
}

#[test]
fn receive_event_outside_interval() {
    let mut collector = Collector::new();

    let event_1 = KeyboardEvent::new(Key::new(32, "path/to/kb/1"), 1, mipoch(0));
    collector.receive(&event_1);
    assert_eq!(collector.pending_cluster().len(), 1);
    assert_eq!(collector.sequence().len(), 0);

    let event_2 = KeyboardEvent::new(Key::new(36, "path/to/kb/2"), 1, mipoch(22));
    collector.receive(&event_2);
    assert_eq!(collector.pending_cluster().len(), 0);
    assert_eq!(collector.sequence().len(), 2);
}

#[test]
fn union_plus_key() {
    let mut collector = Collector::new();

    let event = KeyboardEvent::new(Key::new(32, "path/to/kb/1"), 1, mipoch(0));
    collector.receive(&event);
    assert_eq!(collector.pending_cluster().len(), 1);
    assert_eq!(collector.sequence().len(), 0);

    let event = KeyboardEvent::new(Key::new(33, "path/to/kb/1"), 1, mipoch(10));
    collector.receive(&event);
    assert_eq!(collector.pending_cluster().len(), 2);
    assert_eq!(collector.sequence().len(), 0);

    let event = KeyboardEvent::new(Key::new(36, "path/to/kb/2"), 1, mipoch(100));
    collector.receive(&event);
    assert_eq!(collector.pending_cluster().len(), 0);
    assert_eq!(collector.sequence().len(), 2);
}

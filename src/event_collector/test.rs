use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};

use super::*;

macro_rules! ke {
    ($a:ident $b:ident $c:expr => $d:expr) => {
        KeyboardEvent::new(
            Key::new(stringify!($a), key_code_from_str(stringify!($b)).unwrap()),
            i32_key_state_value_from_str(stringify!($c)),
            mipoch($d),
        )
    };
}

#[test]
fn receive_event_within_interval() {
    let mut collector = Collector::new();

    collector.receive(&ke!(L1 D Down => 0));
    assert!(collector.pending_cluster().len() == 1);

    collector.receive(&ke!(L1 F Down => 4));
    assert!(collector.pending_cluster().len() == 2);
}

#[test]
fn receive_event_outside_interval() {
    let mut collector = Collector::new();

    collector.receive(&ke!(L1 F Down => 0));
    assert!(collector.pending_cluster().len() == 1);
    assert!(collector.sequence().is_empty());

    collector.receive(&ke!(R1 J Down => 22));
    assert!(collector.pending_cluster().is_empty());
    assert!(collector.sequence().len() == 2);
}

#[test]
fn union_plus_key() {
    let mut collector = Collector::new();

    collector.receive(&ke!(L1 D Down => 0));
    assert!(collector.pending_cluster().len() == 1);
    assert!(collector.sequence().is_empty());

    collector.receive(&ke!(L1 F Down => 10));
    assert!(collector.pending_cluster().len() == 2);
    assert!(collector.sequence().is_empty());

    collector.receive(&ke!(R1 J Down => 100));
    assert!(collector.pending_cluster().is_empty());
    assert!(collector.sequence().len() == 2);
}

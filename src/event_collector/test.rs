use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};

use super::*;

macro_rules! ev {
    ( $collector:ident receives: $device:ident $key:ident $state:ident $time:expr =>
      $($checker:ident $value:expr),+ ) => {
        let event = KeyboardEvent::new(
            Key::new(
                stringify!($device),
                key_code_from_str(stringify!($key)).unwrap(),
            ),
            i32_key_state_value_from_str(stringify!($state)),
            mipoch($time),
        );
        $collector.receive(&event);

        $( assert!($collector.$checker().len() == $value); )+
    };
}

#[test]
fn key_down_event_within_interval() {
    let mut collector = Collector::new();

    ev!(collector receives: L1 D Down 0 => sequence 0, pending_cluster 1);
    ev!(collector receives: L1 F Down 4 => sequence 0, pending_cluster 2);
}

#[test]
fn key_down_event_outside_interval() {
    let mut collector = Collector::new();

    ev!(collector receives: L1 D Down  0 => sequence 0, pending_cluster 1);
    ev!(collector receives: R1 J Down 22 => sequence 2, pending_cluster 0);
}

#[test]
fn union_down_then_key_down_later() {
    let mut collector = Collector::new();

    ev!(collector receives: L1 D Down 0   => sequence 0, pending_cluster 1);
    ev!(collector receives: L1 F Down 10  => sequence 0, pending_cluster 2);
    ev!(collector receives: R1 J Down 100 => sequence 2, pending_cluster 0);
}

use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};

use super::*;

macro_rules! event {
    ( $collector:ident receives: $a:ident $b:ident $c:ident $d:expr =>
      $(sequence len $sequence:expr)*, $(pending_cluster len $pending_cluster:expr)* ) => {
        let event = KeyboardEvent::new(
            Key::new(stringify!($a), key_code_from_str(stringify!($b)).unwrap()),
            i32_key_state_value_from_str(stringify!($c)),
            mipoch($d),
        );
        $collector.receive(&event);

        $( assert!( $collector.sequence().len() == $sequence ); )*
        $( assert!( $collector.pending_cluster().len() == $pending_cluster ); )*
    };
}

#[test]
fn key_down_event_within_interval() {
    let mut collector = Collector::new();

    event!(collector receives: L1 D Down 0 => sequence len 0, pending_cluster len 1);
    event!(collector receives: L1 F Down 4 => sequence len 0, pending_cluster len 2);
}

#[test]
fn key_down_event_outside_interval() {
    let mut collector = Collector::new();

    event!(collector receives: L1 D Down 0  => sequence len 0, pending_cluster len 1);
    event!(collector receives: R1 J Down 22 => sequence len 2, pending_cluster len 0);
}

#[test]
fn union_down_then_key_down_later() {
    let mut collector = Collector::new();

    event!(collector receives: L1 D Down 0   => sequence len 0, pending_cluster len 1);
    event!(collector receives: L1 F Down 10  => sequence len 0, pending_cluster len 2);
    event!(collector receives: R1 J Down 100 => sequence len 2, pending_cluster len 0);
}

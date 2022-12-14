use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};

use super::*;

macro_rules! ev{
    ( $collector:ident receives: $a:ident $b:ident $c:ident $d:expr =>
      $(sequence $sequence:expr)*, $(pending_cluster $pending_cluster:expr)* ) => {
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

macro_rules! event {
    ( $variable:ident receives: $device:ident $key:ident $state:ident $time:expr ) => {{
        let event = KeyboardEvent::new(
            Key::new(
                stringify!($device),
                key_code_from_str(stringify!($key)).unwrap(),
            ),
            i32_key_state_value_from_str(stringify!($state)),
            mipoch($time),
        );
        $variable.receive(&event);
    }};
}

macro_rules! check {
    ( $variable:ident $method_1:ident $method_2:ident is $result:expr ) => {
        assert!($variable.$method_1().$method_2() == $result);
    };
}

#[test]
fn key_down_event_within_interval() {
    let mut collector = Collector::new();

    event!(collector receives: L1 D Down 0);
    check!(collector sequence len is 0);
    check!(collector pending_cluster len is 1);

    event!(collector receives: L1 F Down 4);
    check!(collector sequence len is 0);
    check!(collector pending_cluster len is 2);
}

#[test]
fn key_down_event_outside_interval() {
    let mut collector = Collector::new();

    event!(collector receives: L1 D Down 0);
    check!(collector sequence len is 0);
    check!(collector pending_cluster len is 1);

    event!(collector receives: R1 J Down 22);
    check!(collector sequence len is 2);
    check!(collector pending_cluster len is 0);
}

#[test]
fn union_down_then_key_down_later() {
    let mut collector = Collector::new();

    ev!(collector receives: L1 D Down 0   => sequence 0, pending_cluster 1);
    ev!(collector receives: L1 F Down 10  => sequence 0, pending_cluster 2);
    ev!(collector receives: R1 J Down 100 => sequence 2, pending_cluster 0);
}

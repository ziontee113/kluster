use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};

use super::*;

macro_rules! keyboard_event {
    ($device:ident $key:ident $state:ident $time:expr) => {
        KeyboardEvent::new(
            Key::new(
                stringify!($device),
                key_code_from_str(stringify!($key)).unwrap(),
            ),
            i32_key_state_value_from_str(stringify!($state)),
            mipoch($time),
        )
    };
}

macro_rules! ev {
    // collector entry point
    ($collector:ident $($tail:tt)*) => {
        ev!(@ $collector $($tail)*);
    };

    // no-op rule for empty tails
    (@ $collector:ident) => {};

    // block, for using assert! macros
    (@ $collector:ident $_block:block $($tail:tt)*) => {{
        ev!(@ $collector $($tail)*);
    }};

    // collector.receive()
    (@ $collector:ident receives: $d:ident $k:ident $s:ident $t:expr; $($tail:tt)*) => {{
        $collector.receive(&keyboard_event!($d $k $s $t));
        ev!(@ $collector $($tail)*);
    }};

    // assert pending_cluster state: variants with (..)
    // compiler must check this rule first before moving to non (..) variant types
    (@ $collector:ident pending_cluster state: $variant:ident(..) $($tail:tt)*) => {{
        assert!(matches!(
            *$collector.pending_cluster().state(),
            PendingClusterState::$variant(..)
        ));
        ev!(@ $collector $($tail)*);
    }};
    // passert ending_cluster state: variants without (..)
    (@ $collector:ident pending_cluster state: $variant:ident $($tail:tt)*) => {{
        assert!(matches!(
            *$collector.pending_cluster().state(),
            PendingClusterState::$variant
        ));
        ev!(@ $collector $($tail)*);
    }};
}

#[test]
fn my_test() {
    let mut collector = Collector::new();
    ev!(collector
         receives: L1 D Down 0;
         pending_cluster state: Pending

         receives: L1 F Down 4;
         pending_cluster state: Pending

         receives: R1 J Down 22;
         pending_cluster state: Formed(..)
    );
}

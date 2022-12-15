// use crate::test_utilities::{i32_key_state_value_from_str, key_code_from_str, mipoch};
//
// use super::*;
//
// macro_rules! keyboard_event {
//     ($device:ident $key:ident $state:ident $time:expr) => {
//         KeyboardEvent::new(
//             Key::new(
//                 stringify!($device),
//                 key_code_from_str(stringify!($key)).unwrap(),
//             ),
//             i32_key_state_value_from_str(stringify!($state)),
//             mipoch($time),
//         )
//     };
// }
//
// macro_rules! ev {
//     ($collector:ident receives: $device:ident $key:ident $state:ident $time:expr) => {
//         $collector.receive(&keyboard_event!($device $key $state $time));
//     };
//     ( $collector:ident receives: $device:ident $key:ident $state:ident $time:expr =>
//       $($checker:ident $value:expr),+ ) => {
//         $collector.receive(&keyboard_event!($device $key $state $time));
//
//         $( assert!($collector.$checker().len() == $value); )+
//     };
// }

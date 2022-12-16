use std::{
    str::FromStr,
    time::{Duration, SystemTime},
};

use evdev::{EnumParseError, Key};

/// Returns a timestamp elasped `milis` milliseconds from UNIX EPOCH.
/// For testing purposes only.
pub fn mipoch(milis: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
}

/// Uses evdev's `.from_str()` to get u16 key code.
/// For testing purposes only.
pub fn key_code_from_str(key: &str) -> Result<u16, EnumParseError> {
    let mut final_key = key.to_uppercase();

    if !final_key.contains("BTN_") {
        final_key = format!("KEY_{final_key}");
    }

    Ok(Key::from_str(&final_key)?.code())
}

/// Turn &str of "Up" / "Down" / "Hold" to i32
/// For testing purposes only.
pub fn i32_key_state_value_from_str(value: &str) -> i32 {
    match value {
        "Up" => 0,
        "Down" => 1,
        "Hold" => 2,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod keycode_test {
    use super::*;

    #[test]
    fn can_return_u16_keycode_from_str() {
        assert_eq!(key_code_from_str("ESC").unwrap(), 1);
        assert_eq!(key_code_from_str("D").unwrap(), 32);
        assert_eq!(key_code_from_str("j").unwrap(), 36);
        assert_eq!(key_code_from_str("leftctrl").unwrap(), 29);
        assert_eq!(key_code_from_str("BTN_START").unwrap(), 0x13b);
        assert_eq!(key_code_from_str("BTn_sELECT").unwrap(), 0x13a);

        assert!(key_code_from_str("escape").is_err());
        assert!(key_code_from_str("left control").is_err());
        assert!(key_code_from_str("left_control").is_err());
    }
}

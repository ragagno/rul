/// Returns the offset from `b'0'`.
#[inline]
pub const fn offset_from_0(byte: u8) -> u8 {
    return u8::wrapping_sub(byte, b'0');
}

/// Returns the offset from `b'a'`.
#[inline]
pub const fn offset_from_lower_a(byte: u8) -> u8 {
    return u8::wrapping_sub(byte, b'a');
}

/// Returns the offset from `b'W'`.
///
/// This is equivalent, for `b'a'..=b'f'`, to getting the decimal value.
#[inline]
pub const fn hex_offset_from_lower_a(byte: u8) -> u8 {
    return u8::wrapping_sub(byte, b'W');
}

/// Returns the offset from `b'A'`.
#[inline]
pub const fn offset_from_upper_a(byte: u8) -> u8 {
    return u8::wrapping_sub(byte, b'A');
}

/// Returns the offset from `b'7'`.
///
/// This is equivalent, for `b'A'..=b'F'`, to getting the decimal value.
#[inline]
pub const fn hex_offset_from_upper_a(byte: u8) -> u8 {
    return u8::wrapping_sub(byte, b'7');
}

#[cfg(test)]
mod tests {
    use super::*;

    mod assertions {
        #[test]
        fn offset_from_0() {
            assert_eq!(0u8, super::offset_from_0(b'0'));
            assert_eq!(4u8, super::offset_from_0(b'4'));
            assert_eq!(255u8, super::offset_from_0(b'/'));
        }

        #[test]
        fn offset_from_lower_a() {
            assert_eq!(0u8, super::offset_from_lower_a(b'a'));
            assert_eq!(4u8, super::offset_from_lower_a(b'e'));
            assert_eq!(255u8, super::offset_from_lower_a(b'`'));
        }

        #[test]
        fn hex_offset_from_lower_a() {
            assert_eq!(0u8, super::hex_offset_from_lower_a(b'W'));
            assert_eq!(4u8, super::hex_offset_from_lower_a(b'['));
            assert_eq!(10u8, super::hex_offset_from_lower_a(b'a'));
            assert_eq!(15u8, super::hex_offset_from_lower_a(b'f'));
            assert_eq!(255u8, super::hex_offset_from_lower_a(b'V'));
        }

        #[test]
        fn offset_from_upper_a() {
            assert_eq!(0u8, super::offset_from_upper_a(b'A'));
            assert_eq!(4u8, super::offset_from_upper_a(b'E'));
            assert_eq!(255u8, super::offset_from_upper_a(b'@'));
        }

        #[test]
        fn hex_offset_from_upper_a() {
            assert_eq!(0u8, super::hex_offset_from_upper_a(b'7'));
            assert_eq!(4u8, super::hex_offset_from_upper_a(b';'));
            assert_eq!(10u8, super::hex_offset_from_upper_a(b'A'));
            assert_eq!(15u8, super::hex_offset_from_upper_a(b'F'));
            assert_eq!(255u8, super::hex_offset_from_upper_a(b'6'));
        }
    }
}

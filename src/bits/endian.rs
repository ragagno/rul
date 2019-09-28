pub const fn switch_u8(original: u8) -> u8 {
    return original;
}

pub const fn switch_u16(original: u16) -> u16 {
    return ((original & super::masks::u16::BYTE_0) << crate::bits::BITS_PER_BYTE_U16) | ((original & super::masks::u16::BYTE_1) >> crate::bits::BITS_PER_BYTE_U16);
}

pub const fn switch_u32(original: u32) -> u32 {
    return ((original & super::masks::u32::BYTE_0) << 3u32 * crate::bits::BITS_PER_BYTE_U32) | ((original & super::masks::u32::BYTE_1) << crate::bits::BITS_PER_BYTE_U32) | ((original & super::masks::u32::BYTE_2) >> crate::bits::BITS_PER_BYTE_U32) | ((original & super::masks::u32::BYTE_3) >> 3u32 * crate::bits::BITS_PER_BYTE_U32);
}

pub const fn switch_u64(original: u64) -> u64 {
    return ((original & super::masks::u64::BYTE_0) << 7u64 * crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_1) << 5u64 * crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_2) << 3u64 * crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_3) << crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_4) >> crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_5) >> 3u64 * crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_6) >> 5u64 * crate::bits::BITS_PER_BYTE_U64) | ((original & super::masks::u64::BYTE_7) >> 7u64 * crate::bits::BITS_PER_BYTE_U64);
}

pub const fn switch_u128(original: u128) -> u128 {
    return ((original & super::masks::u128::BYTE_0) << 15u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_1) << 13u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_2) << 11u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_3) << 9u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_4) << 7u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_5) << 5u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_6) << 3u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_7) << crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_8) >> crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_9) >> 3u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_10) >> 5u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_11) >> 7u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_12) >> 9u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_13) >> 11u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_14) >> 13u128 * crate::bits::BITS_PER_BYTE_U128) | ((original & super::masks::u128::BYTE_15) >> 15u128 * crate::bits::BITS_PER_BYTE_U128);
}

#[cfg(target_endian = "little")]
pub const fn force_little_u8(original: u8) -> u8 {
    return original;
}

#[cfg(target_endian = "little")]
pub const fn force_little_u16(original: u16) -> u16 {
    return original;
}

#[cfg(target_endian = "little")]
pub const fn force_little_u32(original: u32) -> u32 {
    return original;
}

#[cfg(target_endian = "little")]
pub const fn force_little_u64(original: u64) -> u64 {
    return original;
}

#[cfg(target_endian = "little")]
pub const fn force_little_u128(original: u128) -> u128 {
    return original;
}

#[cfg(target_endian = "big")]
pub const fn force_little_u8(original: u8) -> u8 {
    return switch_u8(original);
}

#[cfg(target_endian = "big")]
pub const fn force_little_u16(original: u16) -> u16 {
    return switch_u16(original);
}

#[cfg(target_endian = "big")]
pub const fn force_little_u32(original: u32) -> u32 {
    return switch_u32(original);
}

#[cfg(target_endian = "big")]
pub const fn force_little_u64(original: u64) -> u64 {
    return switch_u64(original);
}

#[cfg(target_endian = "big")]
pub const fn force_little_u128(original: u128) -> u128 {
    return switch_u128(original);
}

#[cfg(target_endian = "little")]
pub const fn force_big_u8(original: u8) -> u8 {
    return switch_u8(original);
}

#[cfg(target_endian = "little")]
pub const fn force_big_u16(original: u16) -> u16 {
    return switch_u16(original);
}

#[cfg(target_endian = "little")]
pub const fn force_big_u32(original: u32) -> u32 {
    return switch_u32(original);
}

#[cfg(target_endian = "little")]
pub const fn force_big_u64(original: u64) -> u64 {
    return switch_u64(original);
}

#[cfg(target_endian = "little")]
pub const fn force_big_u128(original: u128) -> u128 {
    return switch_u128(original);
}

#[cfg(target_endian = "big")]
pub const fn force_big_u8(original: u8) -> u8 {
    return original;
}

#[cfg(target_endian = "big")]
pub const fn force_big_u16(original: u16) -> u16 {
    return original;
}

#[cfg(target_endian = "big")]
pub const fn force_big_u32(original: u32) -> u32 {
    return original;
}

#[cfg(target_endian = "big")]
pub const fn force_big_u64(original: u64) -> u64 {
    return original;
}

#[cfg(target_endian = "big")]
pub const fn force_big_u128(original: u128) -> u128 {
    return original;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod assertions {
        #[test]
        fn switch_u8() {
            assert_eq!(0x00u8, super::switch_u8(0x00u8));
            assert_eq!(0x01u8, super::switch_u8(0x01u8));
            assert_eq!(0x10u8, super::switch_u8(0x10u8));
        }

        #[test]
        fn switch_u16() {
            assert_eq!(0x00_00u16, super::switch_u16(0x00_00u16));
            assert_eq!(0x01_00u16, super::switch_u16(0x00_01u16));
            assert_eq!(0x10_00u16, super::switch_u16(0x00_10u16));
            assert_eq!(0x00_01u16, super::switch_u16(0x01_00u16));
            assert_eq!(0x00_10u16, super::switch_u16(0x10_00u16));
        }

        #[test]
        fn switch_u32() {
            assert_eq!(0x00_00_00_00u32, super::switch_u32(0x00_00_00_00u32));
            assert_eq!(0x01_00_00_00u32, super::switch_u32(0x00_00_00_01u32));
            assert_eq!(0x10_00_00_00u32, super::switch_u32(0x00_00_00_10u32));
            assert_eq!(0x00_01_00_00u32, super::switch_u32(0x00_00_01_00u32));
            assert_eq!(0x00_10_00_00u32, super::switch_u32(0x00_00_10_00u32));
            assert_eq!(0x00_00_01_00u32, super::switch_u32(0x00_01_00_00u32));
            assert_eq!(0x00_00_10_00u32, super::switch_u32(0x00_10_00_00u32));
            assert_eq!(0x00_00_00_01u32, super::switch_u32(0x01_00_00_00u32));
            assert_eq!(0x00_00_00_10u32, super::switch_u32(0x10_00_00_00u32));
        }

        #[test]
        fn switch_u64() {
            assert_eq!(0x00_00_00_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_00_00_00u64));
            assert_eq!(0x01_00_00_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_00_00_01u64));
            assert_eq!(0x10_00_00_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_00_00_10u64));
            assert_eq!(0x00_01_00_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_00_01_00u64));
            assert_eq!(0x00_10_00_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_00_10_00u64));
            assert_eq!(0x00_00_01_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_01_00_00u64));
            assert_eq!(0x00_00_10_00_00_00_00_00u64, super::switch_u64(0x00_00_00_00_00_10_00_00u64));
            assert_eq!(0x00_00_00_01_00_00_00_00u64, super::switch_u64(0x00_00_00_00_01_00_00_00u64));
            assert_eq!(0x00_00_00_10_00_00_00_00u64, super::switch_u64(0x00_00_00_00_10_00_00_00u64));
            assert_eq!(0x00_00_00_00_01_00_00_00u64, super::switch_u64(0x00_00_00_01_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_10_00_00_00u64, super::switch_u64(0x00_00_00_10_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_01_00_00u64, super::switch_u64(0x00_00_01_00_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_10_00_00u64, super::switch_u64(0x00_00_10_00_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_00_01_00u64, super::switch_u64(0x00_01_00_00_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_00_10_00u64, super::switch_u64(0x00_10_00_00_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_00_00_01u64, super::switch_u64(0x01_00_00_00_00_00_00_00u64));
            assert_eq!(0x00_00_00_00_00_00_00_10u64, super::switch_u64(0x10_00_00_00_00_00_00_00u64));
        }

        #[test]
        fn switch_u128() {
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128));
            assert_eq!(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128));
            assert_eq!(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128));
            assert_eq!(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128));
            assert_eq!(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128));
            assert_eq!(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128));
            assert_eq!(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128));
            assert_eq!(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128));
            assert_eq!(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128, super::switch_u128(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128, super::switch_u128(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128, super::switch_u128(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128, super::switch_u128(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128, super::switch_u128(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128, super::switch_u128(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128, super::switch_u128(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128, super::switch_u128(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128, super::switch_u128(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128, super::switch_u128(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128, super::switch_u128(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
        }

        #[test]
        fn force_little_u8() {
            if cfg!(target_endian = "little") {
                assert_eq!(0x00u8, super::force_little_u8(0x00u8));
                assert_eq!(0x01u8, super::force_little_u8(0x01u8));
                assert_eq!(0x10u8, super::force_little_u8(0x10u8));
            } else {
                assert_eq!(0x00u8, super::force_little_u8(0x00u8));
                assert_eq!(0x01u8, super::force_little_u8(0x01u8));
                assert_eq!(0x10u8, super::force_little_u8(0x10u8));
            }
        }

        #[test]
        fn force_little_u16() {
            if cfg!(target_endian = "little") {
                assert_eq!(0x00_00u16, super::force_little_u16(0x00_00u16));
                assert_eq!(0x00_01u16, super::force_little_u16(0x00_01u16));
                assert_eq!(0x00_10u16, super::force_little_u16(0x00_10u16));
                assert_eq!(0x01_00u16, super::force_little_u16(0x01_00u16));
                assert_eq!(0x10_00u16, super::force_little_u16(0x10_00u16));
            } else {
                assert_eq!(0x00_00u16, super::force_little_u16(0x00_00u16));
                assert_eq!(0x01_00u16, super::force_little_u16(0x00_01u16));
                assert_eq!(0x10_00u16, super::force_little_u16(0x00_10u16));
                assert_eq!(0x00_01u16, super::force_little_u16(0x01_00u16));
                assert_eq!(0x00_10u16, super::force_little_u16(0x10_00u16));
            }
        }

        #[test]
        fn force_little_u32() {
            if cfg!(target_endian = "little") {
                assert_eq!(0x00_00_00_00u32, super::force_little_u32(0x00_00_00_00u32));
                assert_eq!(0x00_00_00_01u32, super::force_little_u32(0x00_00_00_01u32));
                assert_eq!(0x00_00_00_10u32, super::force_little_u32(0x00_00_00_10u32));
                assert_eq!(0x00_00_01_00u32, super::force_little_u32(0x00_00_01_00u32));
                assert_eq!(0x00_00_10_00u32, super::force_little_u32(0x00_00_10_00u32));
                assert_eq!(0x00_01_00_00u32, super::force_little_u32(0x00_01_00_00u32));
                assert_eq!(0x00_10_00_00u32, super::force_little_u32(0x00_10_00_00u32));
                assert_eq!(0x01_00_00_00u32, super::force_little_u32(0x01_00_00_00u32));
                assert_eq!(0x10_00_00_00u32, super::force_little_u32(0x10_00_00_00u32));
            } else {
                assert_eq!(0x00_00_00_00u32, super::force_little_u32(0x00_00_00_00u32));
                assert_eq!(0x01_00_00_00u32, super::force_little_u32(0x00_00_00_01u32));
                assert_eq!(0x10_00_00_00u32, super::force_little_u32(0x00_00_00_10u32));
                assert_eq!(0x00_01_00_00u32, super::force_little_u32(0x00_00_01_00u32));
                assert_eq!(0x00_10_00_00u32, super::force_little_u32(0x00_00_10_00u32));
                assert_eq!(0x00_00_01_00u32, super::force_little_u32(0x00_01_00_00u32));
                assert_eq!(0x00_00_10_00u32, super::force_little_u32(0x00_10_00_00u32));
                assert_eq!(0x00_00_00_01u32, super::force_little_u32(0x01_00_00_00u32));
                assert_eq!(0x00_00_00_10u32, super::force_little_u32(0x10_00_00_00u32));
            }
        }

        #[test]
        fn force_little_u64() {
            if cfg!(target_endian = "little") {
                assert_eq!(0x00_00_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_01u64, super::force_little_u64(0x00_00_00_00_00_00_00_01u64));
                assert_eq!(0x00_00_00_00_00_00_00_10u64, super::force_little_u64(0x00_00_00_00_00_00_00_10u64));
                assert_eq!(0x00_00_00_00_00_00_01_00u64, super::force_little_u64(0x00_00_00_00_00_00_01_00u64));
                assert_eq!(0x00_00_00_00_00_00_10_00u64, super::force_little_u64(0x00_00_00_00_00_00_10_00u64));
                assert_eq!(0x00_00_00_00_00_01_00_00u64, super::force_little_u64(0x00_00_00_00_00_01_00_00u64));
                assert_eq!(0x00_00_00_00_00_10_00_00u64, super::force_little_u64(0x00_00_00_00_00_10_00_00u64));
                assert_eq!(0x00_00_00_00_01_00_00_00u64, super::force_little_u64(0x00_00_00_00_01_00_00_00u64));
                assert_eq!(0x00_00_00_00_10_00_00_00u64, super::force_little_u64(0x00_00_00_00_10_00_00_00u64));
                assert_eq!(0x00_00_00_01_00_00_00_00u64, super::force_little_u64(0x00_00_00_01_00_00_00_00u64));
                assert_eq!(0x00_00_00_10_00_00_00_00u64, super::force_little_u64(0x00_00_00_10_00_00_00_00u64));
                assert_eq!(0x00_00_01_00_00_00_00_00u64, super::force_little_u64(0x00_00_01_00_00_00_00_00u64));
                assert_eq!(0x00_00_10_00_00_00_00_00u64, super::force_little_u64(0x00_00_10_00_00_00_00_00u64));
                assert_eq!(0x00_01_00_00_00_00_00_00u64, super::force_little_u64(0x00_01_00_00_00_00_00_00u64));
                assert_eq!(0x00_10_00_00_00_00_00_00u64, super::force_little_u64(0x00_10_00_00_00_00_00_00u64));
                assert_eq!(0x01_00_00_00_00_00_00_00u64, super::force_little_u64(0x01_00_00_00_00_00_00_00u64));
                assert_eq!(0x10_00_00_00_00_00_00_00u64, super::force_little_u64(0x10_00_00_00_00_00_00_00u64));
            } else {
                assert_eq!(0x00_00_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_00_00u64));
                assert_eq!(0x01_00_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_00_01u64));
                assert_eq!(0x10_00_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_00_10u64));
                assert_eq!(0x00_01_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_01_00u64));
                assert_eq!(0x00_10_00_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_00_10_00u64));
                assert_eq!(0x00_00_01_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_01_00_00u64));
                assert_eq!(0x00_00_10_00_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_00_10_00_00u64));
                assert_eq!(0x00_00_00_01_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_01_00_00_00u64));
                assert_eq!(0x00_00_00_10_00_00_00_00u64, super::force_little_u64(0x00_00_00_00_10_00_00_00u64));
                assert_eq!(0x00_00_00_00_01_00_00_00u64, super::force_little_u64(0x00_00_00_01_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_10_00_00_00u64, super::force_little_u64(0x00_00_00_10_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_01_00_00u64, super::force_little_u64(0x00_00_01_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_10_00_00u64, super::force_little_u64(0x00_00_10_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_01_00u64, super::force_little_u64(0x00_01_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_10_00u64, super::force_little_u64(0x00_10_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_01u64, super::force_little_u64(0x01_00_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_10u64, super::force_little_u64(0x10_00_00_00_00_00_00_00u64));
            }
        }

        #[test]
        fn force_little_u128() {
            if cfg!(target_endian = "little") {
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            } else {
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128));
                assert_eq!(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128));
                assert_eq!(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128));
                assert_eq!(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128));
                assert_eq!(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128));
                assert_eq!(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128));
                assert_eq!(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128));
                assert_eq!(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128));
                assert_eq!(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128, super::force_little_u128(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128, super::force_little_u128(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128, super::force_little_u128(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128, super::force_little_u128(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128, super::force_little_u128(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128, super::force_little_u128(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128, super::force_little_u128(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128, super::force_little_u128(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128, super::force_little_u128(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            }
        }

        #[test]
        fn force_big_u8() {
            if cfg!(target_endian = "big") {
                assert_eq!(0x00u8, super::force_big_u8(0x00u8));
                assert_eq!(0x01u8, super::force_big_u8(0x01u8));
                assert_eq!(0x10u8, super::force_big_u8(0x10u8));
            } else {
                assert_eq!(0x00u8, super::force_big_u8(0x00u8));
                assert_eq!(0x01u8, super::force_big_u8(0x01u8));
                assert_eq!(0x10u8, super::force_big_u8(0x10u8));
            }
        }

        #[test]
        fn force_big_u16() {
            if cfg!(target_endian = "big") {
                assert_eq!(0x00_00u16, super::force_big_u16(0x00_00u16));
                assert_eq!(0x00_01u16, super::force_big_u16(0x00_01u16));
                assert_eq!(0x00_10u16, super::force_big_u16(0x00_10u16));
                assert_eq!(0x01_00u16, super::force_big_u16(0x01_00u16));
                assert_eq!(0x10_00u16, super::force_big_u16(0x10_00u16));
            } else {
                assert_eq!(0x00_00u16, super::force_big_u16(0x00_00u16));
                assert_eq!(0x01_00u16, super::force_big_u16(0x00_01u16));
                assert_eq!(0x10_00u16, super::force_big_u16(0x00_10u16));
                assert_eq!(0x00_01u16, super::force_big_u16(0x01_00u16));
                assert_eq!(0x00_10u16, super::force_big_u16(0x10_00u16));
            }
        }

        #[test]
        fn force_big_u32() {
            if cfg!(target_endian = "big") {
                assert_eq!(0x00_00_00_00u32, super::force_big_u32(0x00_00_00_00u32));
                assert_eq!(0x00_00_00_01u32, super::force_big_u32(0x00_00_00_01u32));
                assert_eq!(0x00_00_00_10u32, super::force_big_u32(0x00_00_00_10u32));
                assert_eq!(0x00_00_01_00u32, super::force_big_u32(0x00_00_01_00u32));
                assert_eq!(0x00_00_10_00u32, super::force_big_u32(0x00_00_10_00u32));
                assert_eq!(0x00_01_00_00u32, super::force_big_u32(0x00_01_00_00u32));
                assert_eq!(0x00_10_00_00u32, super::force_big_u32(0x00_10_00_00u32));
                assert_eq!(0x01_00_00_00u32, super::force_big_u32(0x01_00_00_00u32));
                assert_eq!(0x10_00_00_00u32, super::force_big_u32(0x10_00_00_00u32));
            } else {
                assert_eq!(0x00_00_00_00u32, super::force_big_u32(0x00_00_00_00u32));
                assert_eq!(0x01_00_00_00u32, super::force_big_u32(0x00_00_00_01u32));
                assert_eq!(0x10_00_00_00u32, super::force_big_u32(0x00_00_00_10u32));
                assert_eq!(0x00_01_00_00u32, super::force_big_u32(0x00_00_01_00u32));
                assert_eq!(0x00_10_00_00u32, super::force_big_u32(0x00_00_10_00u32));
                assert_eq!(0x00_00_01_00u32, super::force_big_u32(0x00_01_00_00u32));
                assert_eq!(0x00_00_10_00u32, super::force_big_u32(0x00_10_00_00u32));
                assert_eq!(0x00_00_00_01u32, super::force_big_u32(0x01_00_00_00u32));
                assert_eq!(0x00_00_00_10u32, super::force_big_u32(0x10_00_00_00u32));
            }
        }

        #[test]
        fn force_big_u64() {
            if cfg!(target_endian = "big") {
                assert_eq!(0x00_00_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_01u64, super::force_big_u64(0x00_00_00_00_00_00_00_01u64));
                assert_eq!(0x00_00_00_00_00_00_00_10u64, super::force_big_u64(0x00_00_00_00_00_00_00_10u64));
                assert_eq!(0x00_00_00_00_00_00_01_00u64, super::force_big_u64(0x00_00_00_00_00_00_01_00u64));
                assert_eq!(0x00_00_00_00_00_00_10_00u64, super::force_big_u64(0x00_00_00_00_00_00_10_00u64));
                assert_eq!(0x00_00_00_00_00_01_00_00u64, super::force_big_u64(0x00_00_00_00_00_01_00_00u64));
                assert_eq!(0x00_00_00_00_00_10_00_00u64, super::force_big_u64(0x00_00_00_00_00_10_00_00u64));
                assert_eq!(0x00_00_00_00_01_00_00_00u64, super::force_big_u64(0x00_00_00_00_01_00_00_00u64));
                assert_eq!(0x00_00_00_00_10_00_00_00u64, super::force_big_u64(0x00_00_00_00_10_00_00_00u64));
                assert_eq!(0x00_00_00_01_00_00_00_00u64, super::force_big_u64(0x00_00_00_01_00_00_00_00u64));
                assert_eq!(0x00_00_00_10_00_00_00_00u64, super::force_big_u64(0x00_00_00_10_00_00_00_00u64));
                assert_eq!(0x00_00_01_00_00_00_00_00u64, super::force_big_u64(0x00_00_01_00_00_00_00_00u64));
                assert_eq!(0x00_00_10_00_00_00_00_00u64, super::force_big_u64(0x00_00_10_00_00_00_00_00u64));
                assert_eq!(0x00_01_00_00_00_00_00_00u64, super::force_big_u64(0x00_01_00_00_00_00_00_00u64));
                assert_eq!(0x00_10_00_00_00_00_00_00u64, super::force_big_u64(0x00_10_00_00_00_00_00_00u64));
                assert_eq!(0x01_00_00_00_00_00_00_00u64, super::force_big_u64(0x01_00_00_00_00_00_00_00u64));
                assert_eq!(0x10_00_00_00_00_00_00_00u64, super::force_big_u64(0x10_00_00_00_00_00_00_00u64));
            } else {
                assert_eq!(0x00_00_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_00_00u64));
                assert_eq!(0x01_00_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_00_01u64));
                assert_eq!(0x10_00_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_00_10u64));
                assert_eq!(0x00_01_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_01_00u64));
                assert_eq!(0x00_10_00_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_00_10_00u64));
                assert_eq!(0x00_00_01_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_01_00_00u64));
                assert_eq!(0x00_00_10_00_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_00_10_00_00u64));
                assert_eq!(0x00_00_00_01_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_01_00_00_00u64));
                assert_eq!(0x00_00_00_10_00_00_00_00u64, super::force_big_u64(0x00_00_00_00_10_00_00_00u64));
                assert_eq!(0x00_00_00_00_01_00_00_00u64, super::force_big_u64(0x00_00_00_01_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_10_00_00_00u64, super::force_big_u64(0x00_00_00_10_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_01_00_00u64, super::force_big_u64(0x00_00_01_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_10_00_00u64, super::force_big_u64(0x00_00_10_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_01_00u64, super::force_big_u64(0x00_01_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_10_00u64, super::force_big_u64(0x00_10_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_01u64, super::force_big_u64(0x01_00_00_00_00_00_00_00u64));
                assert_eq!(0x00_00_00_00_00_00_00_10u64, super::force_big_u64(0x10_00_00_00_00_00_00_00u64));
            }
        }

        #[test]
        fn force_big_u128() {
            if cfg!(target_endian = "big") {
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            } else {
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128));
                assert_eq!(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128));
                assert_eq!(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128));
                assert_eq!(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128));
                assert_eq!(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128));
                assert_eq!(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128));
                assert_eq!(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128));
                assert_eq!(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128));
                assert_eq!(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_01_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_01_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_10_00_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_00_10_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_01_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_01_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_10_00_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_00_10_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_01_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_01_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_10_00_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_00_10_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_01_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_01_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_10_00_00_00_00u128, super::force_big_u128(0x00_00_00_00_10_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_01_00_00_00u128, super::force_big_u128(0x00_00_00_01_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_10_00_00_00u128, super::force_big_u128(0x00_00_00_10_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_01_00_00u128, super::force_big_u128(0x00_00_01_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_10_00_00u128, super::force_big_u128(0x00_00_10_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_01_00u128, super::force_big_u128(0x00_01_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_10_00u128, super::force_big_u128(0x00_10_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_01u128, super::force_big_u128(0x01_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
                assert_eq!(0x00_00_00_00_00_00_00_00_00_00_00_00_00_00_00_10u128, super::force_big_u128(0x10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00u128));
            }
        }
    }
}

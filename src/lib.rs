#![no_std]

use core::fmt::{Debug, self};

mod arithmetic;
mod conversion;


/// x = (-1)^(sign) * 2^(exponent - 64) * 1.(mantissa)
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct F16 {
    // TODO: maybe flip sign bit
    bits: u16,
}

impl F16 {
    const SIGN_MASK: u16 = 0b1000_0000_0000_0000;

    const EXPONENT_MASK: u16 = 0b0111_1111_0000_0000;
    const EXPONENT_BIAS: i32 = 64;

    const MANTISSA_MASK: u16 = 0b0000_0000_1111_1111;
    const MANTISSA_BITS: u32 = 8;
    
    
    
    pub const ZERO: Self = Self { bits: 0 };
    pub const MAX: Self = Self { bits: 0b0111_1111_1111_1111 };
    pub const MIN: Self = Self { bits: 0b1111_1111_1111_1111 };
    pub const MIN_POSITIVE: Self = Self { bits: 0b0000_0000_0000_0001 };
    pub const MAX_NEGATIVE: Self = Self { bits: 0b1000_0000_0000_0000 };

    #[must_use]
    pub const fn to_bits(self) -> u16 {
        self.bits
    }

    #[must_use]
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    const fn exponent(self) -> i32 {
        ((self.to_bits() & Self::EXPONENT_MASK) >> Self::MANTISSA_BITS) as i32 - Self::EXPONENT_BIAS
    }

    const fn sign_bit(self) -> u16 {
        self.to_bits() & Self::SIGN_MASK
    }
    
    const fn mantissa_bits(self) -> u16 {
        self.to_bits() & Self::MANTISSA_MASK
    }
}

impl Debug for F16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: this is hacky
        write!(f, "{:?}", f32::from(*self))
    }
}

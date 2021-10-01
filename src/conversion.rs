use crate::F16;

impl From<F16> for f32 {
    fn from(src: F16) -> Self {
        if src == F16::ZERO {
            0.0
        } else {
            let sign_bit = u32::from(src.sign_bit()) << 16;
            #[allow(clippy::cast_sign_loss)]
            let exponent_bits = ((src.exponent() + 127) as u32) << 23;
            let mantissa_bits = u32::from(src.mantissa_bits()) << 15;
            Self::from_bits(sign_bit | exponent_bits | mantissa_bits)
        }
    }
}

impl From<F16> for f64 {
    fn from(src: F16) -> Self {
        if src == F16::ZERO {
            0.0
        } else {
            let sign_bit = u64::from(src.sign_bit()) << 48;
            #[allow(clippy::cast_sign_loss)]
            let exponent_bits = ((src.exponent() + 1023) as u64) << 52;
            let mantissa_bits = u64::from(src.mantissa_bits()) << 44;
            Self::from_bits(sign_bit | exponent_bits | mantissa_bits)
        }
    }
}

// TODO: error handling?
// TODO: handle extreme values, use rounded_bits for bounds checks
impl TryFrom<f32> for F16 {
    type Error = ();

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(src: f32) -> Result<Self, Self::Error> {
        if src.is_finite() {
            if src > Self::MAX_NEGATIVE.into() && src < Self::MIN_POSITIVE.into() {
                return Ok(Self::ZERO);
            } else if src >= Self::MIN.into() && src <= Self::MAX.into() {
                // the exponent is bounded, so overflow is impossible
                let rounded_bits = src.to_bits() + (1 << 14);
                let sign_bit = ((rounded_bits >> 31) as u16) << 15;
                #[allow(clippy::cast_possible_wrap)]
                let exponent = ((rounded_bits >> 23) & 0b1111_1111) as i32 - 127;
                #[allow(clippy::cast_sign_loss)]
                let exponent_bits = ((exponent + Self::EXPONENT_BIAS) as u16) << 8;
                let mantissa_bits = ((rounded_bits >> 15) & 0b1111_1111) as u16;
                return Ok(Self::from_bits(sign_bit | exponent_bits | mantissa_bits));
            }
        }
        Err(())
    }
}

impl TryFrom<f64> for F16 {
    type Error = ();

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(src: f64) -> Result<Self, Self::Error> {
        if src.is_finite() {
            if src > Self::MAX_NEGATIVE.into() && src < Self::MIN_POSITIVE.into() {
                return Ok(Self::ZERO);
            } else if src >= Self::MIN.into() && src <= Self::MAX.into() {
                // the exponent is bounded, so overflow is impossible
                let rounded_bits = src.to_bits() + (1 << 43);
                let sign_bit = ((rounded_bits >> 63) as u16) << 15;
                let exponent = ((rounded_bits >> 52) & 0b0111_1111_1111) as i32 - 1023;
                #[allow(clippy::cast_sign_loss)]
                let exponent_bits = ((exponent + Self::EXPONENT_BIAS) as u16) << 8;
                let mantissa_bits = ((rounded_bits >> 44) & 0b1111_1111) as u16;
                return Ok(Self::from_bits(sign_bit | exponent_bits | mantissa_bits));
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_and_from_f32() {
        for bits in 0..=u16::MAX {
            let f = F16::from_bits(bits);
            assert_eq!(Ok(f), F16::try_from(f32::from(f)), "{:016b}", bits);
        }
    }

    #[test]
    fn into_and_from_f64() {
        for bits in 0..=u16::MAX {
            let f = F16::from_bits(bits);
            assert_eq!(Ok(f), F16::try_from(f64::from(f)), "{:016b}", bits);
        }
    }
}

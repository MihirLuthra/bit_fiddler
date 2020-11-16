/// Sets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::set_bit;
/// 
/// let mut bitmap: u64 = 0;
///
/// set_bit!(bitmap, 0);
/// set_bit!(bitmap, 2);
/// set_bit!(bitmap, 4);
///
/// assert_eq!(bitmap, 0b10101);
/// ```
#[macro_export]
macro_rules! set_bit {
    ($bitmap: ident, $bit_pos: tt) => {
        $bitmap |= 1 << $bit_pos;
    }
}

/// Unsets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::unset_bit;
/// 
/// let mut bitmap: u8 = 0b_1000_0101;
///
/// unset_bit!(bitmap, 0);
/// assert_eq!(bitmap, 0b_1000_0100);
///
/// unset_bit!(bitmap, 2);
/// assert_eq!(bitmap, 0b_1000_0000);
///
/// unset_bit!(bitmap, 7);
/// assert_eq!(bitmap, 0b_0000_0000);
/// ```
#[macro_export]
macro_rules! unset_bit {
    ($bitmap: ident, $bit_pos: tt) => {
        $bitmap &= !(1 << $bit_pos);
    }
}

/// Unsets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::toggle_bit;
/// 
/// let mut bitmap: u8 = 0b_1000_0101;
///
/// toggle_bit!(bitmap, 0);
/// assert_eq!(bitmap, 0b_1000_0100);
///
/// toggle_bit!(bitmap, 1);
/// assert_eq!(bitmap, 0b_1000_0110);
///
/// toggle_bit!(bitmap, 6);
/// assert_eq!(bitmap, 0b_1100_0110);
/// ```
#[macro_export]
macro_rules! toggle_bit {
    ($bitmap: ident, $bit_pos: tt) => {
        $bitmap ^= 1 << $bit_pos;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_bit() {
        let mut bitmap = 0;

        set_bit!(bitmap, 0);
        set_bit!(bitmap, 1);
        set_bit!(bitmap, 2);

        assert_eq!(bitmap, 0b111);
    }

    #[test]
    fn unset_bit() {
        let mut bitmap = 0b0000_0111;

        unset_bit!(bitmap, 0);
        unset_bit!(bitmap, 1);
        unset_bit!(bitmap, 2);

        assert_eq!(bitmap, 0);
    }

    #[test]
    fn toggle_bit() {
        let mut bitmap = 0b0000_0111;

        toggle_bit!(bitmap, 0);
        toggle_bit!(bitmap, 7);
        toggle_bit!(bitmap, 5);

        assert_eq!(bitmap, 0b1010_0110);
    }
}

/// Sets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::set_bit;
/// 
/// let mut bitmap: u64 = 0;
///
/// set_bit!(bitmap, 0);
/// set_bit!(bitmap, 2, 4);
///
/// assert_eq!(bitmap, 0b10101);
///
/// // use `get` as 2nd arg to return the resultant bitmap
///
/// let set_bitmap = set_bit!(bitmap, get, 3);
///
/// assert_eq!(set_bitmap, 0b11101);
/// assert_eq!(bitmap, 0b10101);
///
/// let set_bitmap = set_bit!(0b0011, get, 2, 3);
///
/// assert_eq!(set_bitmap, 0b1111);
/// ```
#[macro_export]
macro_rules! set_bit {
    ($bitmap: tt, get, $( $bit_pos: tt),*) => {
        $bitmap | $( (1 << $bit_pos) | )* 0;
    };
    ($bitmap: ident, $( $bit_pos: tt),* ) => {
        $bitmap |= $( (1 << $bit_pos) | )* 0;
    };
}

/// Unsets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::unset_bit;
/// 
/// let mut bitmap = 0b0000_0111;
/// 
/// unset_bit!(bitmap, 0);
/// unset_bit!(bitmap, 1, 2);
/// 
/// assert_eq!(bitmap, 0);
/// 
/// let mut bitmap = 0b0000_0111;
///
/// // use `get` as 2nd arg to return the resultant bitmap
/// 
/// let res_bitmap = unset_bit!(bitmap, get, 0, 1, 2);
/// 
/// assert_eq!(res_bitmap, 0);
/// assert_eq!(bitmap, 0b0000_0111);
/// 
/// assert_eq!(unset_bit!(0b111, get, 0, 1, 2), 0);
/// ```
#[macro_export]
macro_rules! unset_bit {
    ($bitmap: tt, get, $( $bit_pos: tt),*) => {
        $bitmap & !( $( (1 << $bit_pos) | )* 0 );
    };
    ($bitmap: ident, $( $bit_pos: tt),* ) => {
        $bitmap &= !( $( (1 << $bit_pos) | )* 0 );
    };
}

/// Unsets bit at the given position in bitmap.
///
/// # Example
/// ```
/// use bit_fiddler::toggle_bit;
/// 
/// let mut bitmap = 0b0000_0111;
/// 
/// toggle_bit!(bitmap, 0);
/// toggle_bit!(bitmap, 7, 5);
/// 
/// assert_eq!(bitmap, 0b1010_0110);
/// 
/// let res_bitmap = toggle_bit!(bitmap, get, 0, 1, 2);
/// 
/// assert_eq!(bitmap, 0b1010_0110);
/// assert_eq!(res_bitmap, 0b1010_0001);
/// 
/// assert_eq!(toggle_bit!(0b101, get, 0, 1, 2), 0b010);
/// ```
#[macro_export]
macro_rules! toggle_bit {
    ($bitmap: tt, get, $( $bit_pos: tt),*) => {
        $bitmap ^ ( $( (1 << $bit_pos) | )* 0 );
    };
    ($bitmap: ident, $( $bit_pos: tt),* ) => {
        $bitmap ^= $( (1 << $bit_pos) | )* 0;
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_bit() {
        let mut bitmap: u64 = 0;
		set_bit!(bitmap, 0);
		set_bit!(bitmap, 2, 4);
		
		assert_eq!(bitmap, 0b10101);
		
		// use `get` as 2nd arg to return the set bitmap
		
		let set_bitmap = set_bit!(bitmap, get, 3);
		
		assert_eq!(set_bitmap, 0b11101);
		assert_eq!(bitmap, 0b10101);
		
		let set_bitmap = set_bit!(0b0011, get, 2, 3);
		
		assert_eq!(set_bitmap, 0b1111);
    }

    #[test]
    fn unset_bit() {
        let mut bitmap = 0b0000_0111;

        unset_bit!(bitmap, 0);
        unset_bit!(bitmap, 1, 2);

        assert_eq!(bitmap, 0);

        let mut bitmap = 0b0000_0111;

        let res_bitmap = unset_bit!(bitmap, get, 0, 1, 2);

        assert_eq!(res_bitmap, 0);
        assert_eq!(bitmap, 0b0000_0111);

        assert_eq!(unset_bit!(0b111, get, 0, 1, 2), 0);
    }

    #[test]
    fn toggle_bit() {
        let mut bitmap = 0b0000_0111;

        toggle_bit!(bitmap, 0);
        toggle_bit!(bitmap, 7, 5);

        assert_eq!(bitmap, 0b1010_0110);

        let res_bitmap = toggle_bit!(bitmap, get, 0, 1, 2);

        assert_eq!(bitmap, 0b1010_0110);
        assert_eq!(res_bitmap, 0b1010_0001);

        assert_eq!(toggle_bit!(0b101, get, 0, 1, 2), 0b010);
    }
}

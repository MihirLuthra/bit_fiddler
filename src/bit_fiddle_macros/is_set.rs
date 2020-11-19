/// Macro for checking if single, multiple or range of bits are set.
/// It accepts multiple patterns for different use cases.
/// It doesn't do any overflow or underflow checks. Behaviour on passing
/// invalid args is undefined.
///
/// A common thing in these patterns is `rev`.
/// All patterns support this. Putting `rev` before the
/// bits being checked makes the macro check the bits from left hand side.
/// Without `rev`, bits will be checked from right hand side.
///
/// For example,
///
/// ```
/// # use bit_fiddler::is_set;
/// let mut bitmap: u8 = 0b_0001_0100;
///
/// // Checking 2nd bit from rhs
/// let res = is_set!(bitmap, 2);
/// assert_eq!(res, true);
///
/// // Unsetting 3rd bit from lhs
/// let res = is_set!(bitmap, rev 3);
/// assert_eq!(res, true);
/// ```
/// # Checking Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for checking range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::is_set;
///
/// // Check third bit from the right.
/// let bitmap = 0b100;
/// let res = is_set!(bitmap, 2);
/// assert_eq!(res, true);
///
/// // Check third bit from the left.
/// let bitmap: u8 = 0b_0000_0100;
/// let res = is_set!(bitmap, rev 2);
/// assert_eq!(res, false);
///
/// // Check second, third & fourth bit from the right.
/// let bitmap = 0b1010;
/// let res = is_set!(bitmap, [1, 2, 3]);
/// assert_eq!(res, false); // as third bit is not set
///
/// // Check second, third & fourth bit from the left.
/// let bitmap: u8 = 0b0111_0000;
/// let res = is_set!(bitmap, rev [1, 2, 3]);
/// assert_eq!(res, true);
///
/// // Check second & third (1 and 2) bit from the right.
/// let bitmap = 0b110;
/// let res = is_set!(bitmap, [1..3]);
/// assert_eq!(res, true);
///
/// // Starting from second bit, check 2 bits from the right.
/// let bitmap = 0b110;
/// let res = is_set!(bitmap, [start = 1, count = 2]);
/// assert_eq!(res, true);
///
/// // Check second & third bit (1 and 2) from the left.
/// let bitmap: u8 = 0b_0110_0000;
/// let res = is_set!(bitmap, rev [1..3]);
/// assert_eq!(res, true);
///
/// // Starting from second bit, check 2 bits from the left.
/// let bitmap: u8 = 0b_0110_0000;
/// let res = is_set!(bitmap, rev [start = 1, count = 2]);
/// assert_eq!(res, true);
/// ```
#[macro_export]
macro_rules! is_set { 
    // let bitmap = 0b1110;
    // let res = is_set!(bitmap, [1, 2, 3]);
    // assert_eq!(res, true);
    ($bitmap: tt, [$( $bit_pos: tt),*]) => {
        {
            let bits_to_check = ($( (1 << $bit_pos) | )* 0);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b0111_0000;
    // let res = is_set!(bitmap, rev [1, 2, 3]);
    // assert_eq!(res, true);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            let total_bit_count = $crate::max_bits!($bitmap);
            let bits_to_check = ($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap = 0b110;
    // let res = is_set!(bitmap, [1..3]);
    // assert_eq!(res, true);
    ($bitmap: tt, [$start_pos: tt .. $end_pos: tt]) => {
        {
            let count_to_set = $end_pos - $start_pos;
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
            let bits_to_check = (mask << $start_pos);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap = 0b110;
    // let res = is_set!(bitmap, [start = 1, count = 2]);
    // assert_eq!(res, true);
    ($bitmap: tt, [start = $start_pos: tt, count = $count: tt]) => {
        {
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..$count], bit_count = total_bit_count);
            let bits_to_check = (mask << $start_pos);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let res = is_set!(bitmap, rev [1..3]);
    // assert_eq!(res, true);
    ($bitmap: tt, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            let count_to_set = $end_pos - $start_pos;
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
            let bits_to_check 
                = (mask << (total_bit_count - $start_pos - 1 - (count_to_set - 1)));
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let res = is_set!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(res, true);
    ($bitmap: tt, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..$count], bit_count = total_bit_count);
            let bits_to_check
                = (mask << (total_bit_count - $start_pos - 1 - ($count - 1)));
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0010_0000;
    // let res = is_set!(bitmap, rev 2);
    // assert_eq!(res, true);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            let total_bit_count = $crate::max_bits!($bitmap);
            ( $bitmap & (1 << (total_bit_count - $bit_pos - 1)) )
                != 0
        }
    };

    // let bitmap = 0b100;
    // let res = is_set!(bitmap, 2);
    // assert_eq!(res, true);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            ( $bitmap & (1 << $bit_pos) )
                != 0
        }
    };
}

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
/// let res = is_set!(bitmap, u8, 2);
/// assert_eq!(res, true);
///
/// // Unsetting 3rd bit from lhs
/// let res = is_set!(bitmap, u8, rev 3);
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
/// let res = is_set!(bitmap, u8, 2);
/// assert_eq!(res, true);
///
/// // Check third bit from the left.
/// let bitmap: u8 = 0b_0000_0100;
/// let res = is_set!(bitmap, u8, rev 2);
/// assert_eq!(res, false);
///
/// // Check second, third & fourth bit from the right.
/// let bitmap = 0b1010;
/// let res = is_set!(bitmap, u8, [1, 2, 3]);
/// assert_eq!(res, false); // as third bit is not set
///
/// // Check second, third & fourth bit from the left.
/// let bitmap: u8 = 0b0111_0000;
/// let res = is_set!(bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(res, true);
///
/// // Check second & third (1 and 2) bit from the right.
/// let bitmap = 0b110;
/// let res = is_set!(bitmap, u8, [1..3]);
/// assert_eq!(res, true);
///
/// // Starting from second bit, check 2 bits from the right.
/// let bitmap = 0b110;
/// let res = is_set!(bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(res, true);
///
/// // Check second & third bit (1 and 2) from the left.
/// let bitmap: u8 = 0b_0110_0000;
/// let res = is_set!(bitmap, u8, rev [1..3]);
/// assert_eq!(res, true);
///
/// // Starting from second bit, check 2 bits from the left.
/// let bitmap: u8 = 0b_0110_0000;
/// let res = is_set!(bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(res, true);
/// ```
#[macro_export]
macro_rules! is_set {
    ($bitmap: tt, $ty: ty, [..]) => {
        {
            $bitmap == !0
        }
    };

    ($bitmap: tt, $ty: ty, rev [..]) => {
        {
            $bitmap == !0
        }
    };

    ($bitmap: tt, $ty: ty, [$( $bit_pos: tt),*]) => {
        {
            let bits_to_check = ($( ((1 as $ty) << $bit_pos) | )* (0 as $ty));
            (($bitmap as $ty) & bits_to_check) == bits_to_check
        }
    };

    ($bitmap: tt, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        {
            let total_bit_count = $crate::max_bits!(type = ($ty));
            let bits_to_check = ($( ((1 as $ty) << (total_bit_count - $bit_pos - 1)) | )* (0 as $ty));
            (($bitmap as $ty) & bits_to_check) == bits_to_check
        }
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([$start_pos..$end_pos], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!([$start_pos..], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([..$end_pos], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!([start = $start_pos, count = $count], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [$start_pos..$end_pos], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!(rev [$start_pos..], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [..$end_pos], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!(rev [start = $start_pos, count = $count], ($ty));
            ($bitmap & mask) == mask
        }
    };

    ($bitmap: tt, $ty: ty, rev $bit_pos: tt) => {
        {
            let total_bit_count = $crate::max_bits!(type = ($ty));
            ( ($bitmap as $ty) & ((1 as $ty) << (total_bit_count - $bit_pos - 1)) )
                != (0 as $ty)
        }
    };

    ($bitmap: tt, $ty: ty, $bit_pos: tt) => {
        {
            ( ($bitmap as $ty) & ((1 as $ty) << $bit_pos) )
                != (0 as $ty)
        }
    };
}

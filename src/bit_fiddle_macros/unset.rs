/// Macro for usetting single, multiple or range of bits.
/// It accepts multiple patterns for different use cases.
/// It doesn't do any overflow or underflow checks. Behaviour on passing
/// invalid args is undefined.
///
/// For all patterns, first arg is the bitmap.
/// Bitmap can be passed in 2 ways:
///
/// In first case, it will return the resultant bitmap after unsetting bits
/// without making any changes to the bitmap passed.
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// let bitmap: u8 = 0b_1111_1111;
/// let res = unset!(bitmap, u8, ...);
/// assert_eq!(bitmap, 0b_1111_1111); // Stays same
/// println!("{:#b}", res); // new bitmap with bits unset
/// # }}
/// ```
///
/// In this case, bitmap can also be a literal:
///
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// // bits will be unset on 0b_0000_0101 and moved to res
/// let res = unset!(0b_0000_0101, ...);
/// # }}
/// ```
///
/// In the second case, we use `in` before the bitmap.
/// This pattern will make changes to the bitmap itself and
/// return nothing.
///
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// let mut bitmap = 0b_1111_1111;
/// unset!(in bitmap, u8, ...);
/// // bits were unset in bitmap
/// # }}
/// ```
/// Another common thing in these patterns is `rev`.
/// All patterns support this. Putting `rev` before the
/// bits being unset makes the macro unset the bits from left hand side.
/// Without `rev`, bits will be unset from right hand side.
///
/// For example,
///
/// ```
/// # use bit_fiddler::unset;
/// let mut bitmap: u8 = 0b_0010_0100;
///
/// // Unsetting 2nd bit from rhs
/// unset!(in bitmap, u8, 2);
/// assert_eq!(bitmap, 0b_0010_0000);
///
/// // Unsetting 2nd bit from lhs
/// unset!(in bitmap, u8, rev 2);
/// assert_eq!(bitmap, 0b_0000_0000);
/// ```
/// # Unsetting Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for unsetting range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::unset;
///
/// // Unset third bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = unset!(bitmap, u8, 2);
/// assert_eq!(x, 0);
///
/// // Unset third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// unset!(in bitmap, u8, 2);
/// assert_eq!(bitmap, 0);
///
/// // Unset third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0010_0000;
/// let x = unset!(bitmap, u8, rev 2);
/// assert_eq!(x, 0);
///
/// // Unset third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0010_0000;
/// unset!(in bitmap, u8, rev 2);
/// assert_eq!(bitmap, 0);
///
/// // Unset second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0b1110;
/// let x = unset!(bitmap, u8, [1, 2, 3]);
/// assert_eq!(x, 0);
///
/// // Unset second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b1110;
/// unset!(in bitmap, u8, [1, 2, 3]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b0111_0000;
/// let x = unset!(bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(x, 0);
///
/// // Unset second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b0111_0000;
/// unset!(in bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = unset!(bitmap, u8, [1..3]);
/// assert_eq!(x, 0);
///
/// // Unset second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// unset!(in bitmap, u8, [1..3]);
/// assert_eq!(bitmap, 0);
///
/// // Starting from second bit, unset 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = unset!(bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, unset 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// unset!(in bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = unset!(bitmap, u8, rev [1..3]);
/// assert_eq!(x, 0);
///
/// // Unset second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// unset!(in bitmap, u8, rev [1..3]);
/// assert_eq!(bitmap, 0);
///
/// // Starting from second bit, unset 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = unset!(bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, unset 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// unset!(in bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// ```
#[macro_export]
macro_rules! unset {
    ($bitmap: tt, $ty: ty, [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty) & !($( ((1 as $ty) << $bit_pos) | )* (0 as $ty))
        }
    };

    (in $bitmap: ident, $ty: ty, [$( $bit_pos: tt),*]) => {
        $bitmap &= !($( ((1 as $ty) << $bit_pos) | )* (0 as $ty));
    };

    ($bitmap: tt, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty)
                & !($( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty))
        }
    };

    (in $bitmap: ident, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        $bitmap &= !($( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty));
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
            $bitmap & !mask
        }
    };

    (in $bitmap: ident, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
        $bitmap &= !mask;
    };

    ($bitmap: tt, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!([start = ($start_pos), count = ($count)], ($ty));
            $bitmap & !mask
        }
    };

    (in $bitmap: ident, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!([start = ($start_pos), count = ($count)], $ty);
        $bitmap &= !mask;
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
            $bitmap & !mask
        }
    };

    (in $bitmap: ident, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
        $bitmap &= !mask;
    };

    ($bitmap: tt, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
            $bitmap & !mask
        }
    };

    (in $bitmap: ident, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
        $bitmap &= !mask;
    };

    ($bitmap: tt, $ty: ty, rev $bit_pos: tt) => {
        {
            ($bitmap as $ty) & !((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1))
        }
    };

    (in $bitmap: ident, $ty: ty, rev $bit_pos: tt) => {
        $bitmap &= !((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1));
    };

    ($bitmap: tt, $ty: ty, $bit_pos: tt) => {
        {
            ($bitmap as $ty) & !((1 as $ty) << $bit_pos)
        }
    };

    (in $bitmap: ident, $ty: ty, $bit_pos: tt) => {
        $bitmap &= !((1 as $ty) << $bit_pos);
    };
}

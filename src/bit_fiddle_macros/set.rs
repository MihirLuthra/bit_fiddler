/// Macro for setting single, multiple or range of bits.
/// It accepts multiple patterns for different use cases.
/// It doesn't do any overflow or underflow checks. Behaviour on passing
/// invalid args is undefined.
///
/// For all patterns, first arg is the bitmap.
/// Bitmap can be passed in 2 ways:
///
/// In first case, it will return the resultant bitmap after setting bits
/// without making any changes to the bitmap passed.
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// let bitmap: u8 = 0b_0000_0000;
/// let res = set!(bitmap, u8, ...);
/// assert_eq!(bitmap, 0b_0000_0000); // Stays same
/// println!("{:#b}", res); // new bitmap with set bits
/// # }}
/// ```
///
/// In this case, bitmap can also be a literal:
///
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// // bits will be set on 0b_0000_0101 and moved to res
/// let res = set!(0b_0000_0101, u8, ...);
/// # }}
/// ```
///
/// In the second case, we use `in` before the bitmap.
/// This pattern will make changes to the bitmap itself and
/// return nothing.
///
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// let mut bitmap = 0b_0000_0000;
/// set!(in bitmap, u8, ...);
/// // bits were set in bitmap
/// # }}
/// ```
/// Another common thing in these patterns is `rev`.
/// All patterns support this. Putting `rev` before the
/// bits being set makes the macro set the bits from left hand side.
/// Without `rev`, bits will be set from right hand side.
///
/// For example,
///
/// ```
/// # use bit_fiddler::set;
/// let mut bitmap = 0;
///
/// // Setting 2nd bit from rhs
/// set!(in bitmap, u8, 2);
/// assert_eq!(bitmap, 0b_0000_0100);
///
/// // Setting 2nd bit from lhs
/// set!(in bitmap, u8, rev 2);
/// assert_eq!(bitmap, 0b_0010_0100);
/// ```
///
/// # Examples
/// ```
/// use bit_fiddler::set;
///
/// // Set third bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set!(bitmap, u8, 2);
/// assert_eq!(x, 0b100);
///
/// // Set third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set!(in bitmap, u8, 2);
/// assert_eq!(bitmap, 0b100);
///
/// // Set third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set!(bitmap, u8, rev 2);
/// assert_eq!(x, 0b_0010_0000);
///
/// // Set third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set!(in bitmap, u8, rev 2);
/// assert_eq!(bitmap, 0b_0010_0000);
///
/// // Set second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set!(bitmap, u8, [1, 2, 3]);
/// assert_eq!(x, 0b1110);
///
/// // Set second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set!(in bitmap, u8, [1, 2, 3]);
/// assert_eq!(bitmap, 0b1110);
///
/// // Set second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set!(bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(x, 0b0111_0000);
///
/// // Set second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set!(in bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0b0111_0000);
///
/// // Set second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set!(bitmap, u8, [1..3]);
/// assert_eq!(x, 0b110);
///
/// // Set second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set!(in bitmap, u8, [1..3]);
/// assert_eq!(bitmap, 0b110);
///
/// // Starting from second bit, set 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set!(bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(x, 0b110);
///
/// // Starting from second bit, set 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set!(in bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0b110);
///
/// // Set second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set!(bitmap, u8, rev [1..3]);
/// assert_eq!(x, 0b_0110_0000);
///
/// // Set second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set!(in bitmap, u8, rev [1..3]);
/// assert_eq!(bitmap, 0b_0110_0000);
///
/// // Starting from second bit, set 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set!(bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(x, 0b_0110_0000);
///
/// // Starting from second bit, set 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set!(in bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0b_0110_0000);
///
/// ```
#[macro_export]
macro_rules! set {
    ($bitmap: tt, $ty: ty, [..]) => {
        {
            !(0 as $ty)
        }
    };

    (in $bitmap: ident, $ty: ty, [..]) => {
        $bitmap |= !(0 as $ty);
    };

    ($bitmap: tt, $ty: ty, rev [..]) => {
        {
            !(0 as $ty)
        }
    };

    (in $bitmap: ident, $ty: ty, rev [..]) => {
        $bitmap |= !(0 as $ty);
    };

    ($bitmap: tt, $ty: ty, [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty) | $( ((1 as $ty) << $bit_pos) | )* (0 as $ty)
        }
    };

    (in $bitmap: ident, $ty: ty, [$( $bit_pos: tt),*]) => {
        $bitmap |= $( ((1 as $ty) << $bit_pos) | )* (0 as $ty);
    };

    ($bitmap: tt, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty)
                | $( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty)
        }
    };

    (in $bitmap: ident, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        $bitmap |= $( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty);
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
            $bitmap | mask
        }
    };

    ($bitmap: tt, $ty: ty, [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([..($end_pos)], ($ty));
            $bitmap | mask
        }
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!([($start_pos)..], ($ty));
            $bitmap | mask
        }
    };

    (in $bitmap: ident, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
        $bitmap |= mask;
    };

    (in $bitmap: ident, $ty: ty, [.. $end_pos: tt]) => {
        let mask = $crate::mask!([..($end_pos)], ($ty));
        $bitmap |= mask;
    };

    (in $bitmap: ident, $ty: ty, [$start_pos: tt ..]) => {
        let mask = $crate::mask!([($start_pos)..], ($ty));
        $bitmap |= mask;
    };
 
    ($bitmap: tt, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!([start = ($start_pos), count = ($count)], ($ty));
            $bitmap | mask
        }
    };

    (in $bitmap: ident, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!([start = ($start_pos), count = ($count)], $ty);
        $bitmap |= mask;
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
            $bitmap | mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [..($end_pos)], ($ty));
            $bitmap | mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!(rev [($start_pos)..], ($ty));
            $bitmap | mask
        }
    };
 
    (in $bitmap: ident, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
        $bitmap |= mask;
    };

    (in $bitmap: ident, $ty: ty, rev [.. $end_pos: tt]) => {
        let mask = $crate::mask!(rev [..($end_pos)], ($ty));
        $bitmap |= mask;
    };

    (in $bitmap: ident, $ty: ty, rev [$start_pos: tt ..]) => {
        let mask = $crate::mask!(rev [($start_pos)..], ($ty));
        $bitmap |= mask;
    };
 
    ($bitmap: tt, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
            $bitmap | mask
        }
    };

    (in $bitmap: ident, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
        $bitmap |= mask;
    };

    ($bitmap: tt, $ty: ty, rev $bit_pos: tt) => {
        {
            ($bitmap as $ty) | ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1))
        }
    };

    (in $bitmap: ident, $ty: ty, rev $bit_pos: tt) => {
        $bitmap |= ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1));
    };

    ($bitmap: tt, $ty: ty, $bit_pos: tt) => {
        {
            ($bitmap as $ty) | ((1 as $ty) << $bit_pos)
        }
    };

    (in $bitmap: ident, $ty: ty, $bit_pos: tt) => {
        $bitmap |= (1 as $ty) << $bit_pos;
    };
}

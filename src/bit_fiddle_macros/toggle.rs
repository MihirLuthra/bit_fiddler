/// Macro for toggling single, multiple or range of bits.
/// It accepts multiple patterns for different use cases.
/// It doesn't do any overflow or underflow checks. Behaviour on passing
/// invalid args is undefined.
///
/// For all patterns, first arg is the bitmap.
/// Bitmap can be passed in 2 ways:
///
/// In first case, it will return the resultant bitmap after toggling bits
/// without making any changes to the bitmap passed.
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// let bitmap: u8 = 0b_1010_1010;
/// let res = toggle!(bitmap, u8, ...);
/// assert_eq!(bitmap, 0b_1010_1010); // Stays same
/// println!("{:#b}", res); // new bitmap with bits unset
/// # }}
/// ```
///
/// In this case, bitmap can also be a literal:
///
/// ```
/// # macro_rules! dont_test_but_have_syntax_highlighting { () => {
/// // bits will be toggled on 0b_0000_0101 and moved to res
/// let res = toggle!(0b_0000_0101, ...);
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
/// toggle!(in bitmap, u8, ...);
/// // bits were toggled in bitmap
/// # }}
/// ```
/// Another common thing in these patterns is `rev`.
/// All patterns support this. Putting `rev` before the
/// bits being toggled makes the macro toggle the bits from left hand side.
/// Without `rev`, bits will be toggled from right hand side.
///
/// For example,
///
/// ```
/// # use bit_fiddler::toggle;
/// let mut bitmap: u8 = 0b_0010_0100;
///
/// // Toggling 2nd and 3rd bit from rhs
/// toggle!(in bitmap, u8, [2, 3]);
/// assert_eq!(bitmap, 0b_0010_1000);
///
/// // Unsetting 2nd and 3rd bit from lhs
/// toggle!(in bitmap, u8, rev [2, 3]);
/// assert_eq!(bitmap, 0b_0001_1000);
/// ```
///
/// # Examples
/// ```
/// use bit_fiddler::toggle;
///
/// // Toggle third bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle!(bitmap, u8, 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle!(in bitmap, u8, 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0010_0000;
/// let x = toggle!(bitmap, u8, rev 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0010_0000;
/// toggle!(in bitmap, u8, rev 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0b1010;
/// let x = toggle!(bitmap, u8, [1, 2, 3]);
/// assert_eq!(x, 0b0100);
///
/// // Toggle second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b1010;
/// toggle!(in bitmap, u8, [1, 2, 3]);
/// assert_eq!(bitmap, 0b0100);
///
/// // Toggle second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b0101_0000;
/// let x = toggle!(bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(x, 0b0010_0000);
///
/// // Toggle second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b0101_0000;
/// toggle!(in bitmap, u8, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0b0010_0000);
///
/// // Toggle second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle!(bitmap, u8, [1..3]);
/// assert_eq!(x, 0b010);
///
/// // Toggle second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle!(in bitmap, u8, [1..3]);
/// assert_eq!(bitmap, 0b010);
///
/// // Starting from second bit, toggle 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = toggle!(bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// toggle!(in bitmap, u8, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle!(bitmap, u8, rev [1..3]);
/// assert_eq!(x, 0);
///
/// // Toggle second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle!(in bitmap, u8, rev [1..3]);
/// assert_eq!(bitmap, 0);
///
/// // Starting from second bit, toggle 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle!(bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle!(in bitmap, u8, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// ```
#[macro_export]
macro_rules! toggle {
    ($bitmap: tt, $ty: ty, [..]) => {
        {
            !($bitmap)
        }
    };

    (in $bitmap: ident, $ty: ty, [..]) => {
        {
            $bitmap = !($bitmap);
        }
    };

    ($bitmap: tt, $ty: ty, rev [..]) => {
        {
            !($bitmap)
        }
    };

    (in $bitmap: ident, $ty: ty, rev [..]) => {
        {
            $bitmap = !($bitmap);
        }
    };

    ($bitmap: tt, $ty: ty, [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty) ^ ($( ((1 as $ty) << $bit_pos) | )* (0 as $ty))
        }
    };

    (in $bitmap: ident, $ty: ty, [$( $bit_pos: tt),*]) => {
        $bitmap ^= $( ((1 as $ty) << $bit_pos) | )* (0 as $ty);
    };

    ($bitmap: tt, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        {
            ($bitmap as $ty)
                ^ ($( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty))
        }
    };

    (in $bitmap: ident, $ty: ty, rev [$( $bit_pos: tt),*]) => {
        $bitmap ^= $( ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1)) | )* (0 as $ty);
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
            $bitmap ^ mask
        }
    };

    ($bitmap: tt, $ty: ty, [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!([..($end_pos)], ($ty));
            $bitmap ^ mask
        }
    };

    ($bitmap: tt, $ty: ty, [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!([($start_pos)..], ($ty));
            $bitmap ^ mask
        }
    };

    (in $bitmap: ident, $ty: ty, [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!([($start_pos)..($end_pos)], ($ty));
        $bitmap ^= mask;
    };

    (in $bitmap: ident, $ty: ty, [$start_pos: tt ..]) => {
        let mask = $crate::mask!([($start_pos)..], ($ty));
        $bitmap ^= mask;
    };

    (in $bitmap: ident, $ty: ty, [.. $end_pos: tt]) => {
        let mask = $crate::mask!([..($end_pos)], ($ty));
        $bitmap ^= mask;
    };

    ($bitmap: tt, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!([start = ($start_pos), count = ($count)], ($ty));
            $bitmap ^ mask
        }
    };

    (in $bitmap: ident, $ty: ty, [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!([start = ($start_pos), count = ($count)], $ty);
        $bitmap ^= mask;
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
            $bitmap ^ mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [.. $end_pos: tt]) => {
        {
            let mask = $crate::mask!(rev [..($end_pos)], ($ty));
            $bitmap ^ mask
        }
    };

    ($bitmap: tt, $ty: ty, rev [$start_pos: tt ..]) => {
        {
            let mask = $crate::mask!(rev [($start_pos)..], ($ty));
            $bitmap ^ mask
        }
    };

    (in $bitmap: ident, $ty: ty, rev [$start_pos: tt .. $end_pos: tt]) => {
        let mask = $crate::mask!(rev [($start_pos)..($end_pos)], ($ty));
        $bitmap ^= mask;
    };

    (in $bitmap: ident, $ty: ty, rev [$start_pos: tt ..]) => {
        let mask = $crate::mask!(rev [($start_pos)..], ($ty));
        $bitmap ^= mask;
    };

    (in $bitmap: ident, $ty: ty, rev [.. $end_pos: tt]) => {
        let mask = $crate::mask!(rev [..($end_pos)], ($ty));
        $bitmap ^= mask;
    };

    ($bitmap: tt, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
            $bitmap ^ mask
        }
    };

    (in $bitmap: ident, $ty: ty, rev [start = $start_pos: tt, count = $count: tt]) => {
        let mask = $crate::mask!(rev [start = ($start_pos), count = ($count)], ($ty));
        $bitmap ^= mask;
    };

    ($bitmap: tt, $ty: ty, rev $bit_pos: tt) => {
        {
            ($bitmap as $ty) ^ ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1))
        }
    };

    (in $bitmap: ident, $ty: ty, rev $bit_pos: tt) => {
        $bitmap ^= ((1 as $ty) << ($crate::max_bits!(type = ($ty)) - $bit_pos - 1));
    };

    ($bitmap: tt, $ty: ty, $bit_pos: tt) => {
        {
            ($bitmap as $ty) ^ ((1 as $ty) << $bit_pos)
        }
    };

    (in $bitmap: ident, $ty: ty, $bit_pos: tt) => {
        $bitmap ^= (1 as $ty) << $bit_pos;
    };
}

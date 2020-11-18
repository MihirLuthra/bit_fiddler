use super::{check_bitmap_impl, bitmap_trait::{Bitmap}};

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
/// let res = set_bmp!(bitmap, ...);
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
/// let res = set_bmp!(0b_0000_0101, ...);
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
/// set_bmp!(in bitmap, ...);
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
/// # use bit_fiddler::set_bmp;
/// let mut bitmap: u8 = 0;
///
/// // Setting 2nd bit from rhs
/// set_bmp!(in bitmap, 2);
/// assert_eq!(bitmap, 0b_0000_0100);
///
/// // Setting 2nd bit from lhs
/// set_bmp!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0b_0010_0100);
/// ```
///
/// # Setting Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for setting range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::set_bmp;
///
/// // Set third bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set_bmp!(bitmap, 2);
/// assert_eq!(x, 0b100);
///
/// // Set third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set_bmp!(in bitmap, 2);
/// assert_eq!(bitmap, 0b100);
///
/// // Set third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set_bmp!(bitmap, rev 2);
/// assert_eq!(x, 0b_0010_0000);
///
/// // Set third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set_bmp!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0b_0010_0000);
///
/// // Set second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set_bmp!(bitmap, [1, 2, 3]);
/// assert_eq!(x, 0b1110);
///
/// // Set second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set_bmp!(in bitmap, [1, 2, 3]);
/// assert_eq!(bitmap, 0b1110);
///
/// // Set second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set_bmp!(bitmap, rev [1, 2, 3]);
/// assert_eq!(x, 0b0111_0000);
///
/// // Set second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set_bmp!(in bitmap, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0b0111_0000);
///
/// // Set second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set_bmp!(bitmap, [1..3]);
/// assert_eq!(x, 0b110);
///
/// // Set second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set_bmp!(in bitmap, [1..3]);
/// assert_eq!(bitmap, 0b110);
///
/// // Starting from second bit, set 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0;
/// let x = set_bmp!(bitmap, [start = 1, count = 2]);
/// assert_eq!(x, 0b110);
///
/// // Starting from second bit, set 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0;
/// set_bmp!(in bitmap, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0b110);
///
/// // Set second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set_bmp!(bitmap, rev [1..3]);
/// assert_eq!(x, 0b_0110_0000);
///
/// // Set second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set_bmp!(in bitmap, rev [1..3]);
/// assert_eq!(bitmap, 0b_0110_0000);
/// 
/// // Starting from second bit, set 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0;
/// let x = set_bmp!(bitmap, rev [start = 1, count = 2]);
/// assert_eq!(x, 0b_0110_0000);
///
/// // Starting from second bit, set 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0;
/// set_bmp!(in bitmap, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0b_0110_0000);
/// 
/// ```
#[macro_export]
macro_rules! set_bmp { 
    // let bitmap = 0;
    // let x = set_bmp!(bitmap, [1, 2, 3]);
    // assert_eq!(x, 0b1110);
    ($bitmap: tt, [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap | $( (1 << $bit_pos) | )* 0
        }
    };
    // let mut bitmap = 0;
    // set_bmp!(in bitmap, [1, 2, 3]);
    // assert_eq!(bitmap, 0b1110);
    (in $bitmap: ident, [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap |= $( (1 << $bit_pos) | )* 0;
    };

    // let bitmap: u8 = 0;
    // let x = set_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(x, 0b0111_0000);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap | $( (1 << (total_bit_count - $bit_pos - 1)) | )* 0
        }
    };
    // let mut bitmap: u8 = 0;
    // set_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(bitmap, 0b0111_0000);
    (in $bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap |= $( (1 << (total_bit_count - $bit_pos - 1)) | )* 0;
    };

    // let bitmap = 0;
    // let x = set_bmp!(bitmap, [1..3]);
    // assert_eq!(x, 0b110);
    ($bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            $bitmap | ((1 << count_to_set) - 1) << $start_pos
        }
    };

    // let mut bitmap = 0;
    // set_bmp!(in bitmap, [1..3]);
    // assert_eq!(bitmap, 0b110);
    (in $bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let count_to_set = $end_pos - $start_pos;
        $bitmap |= ((1 << count_to_set) - 1) << $start_pos;
    };

    // let bitmap = 0;
    // let x = set_bmp!(bitmap, [start = 1, count = 2]);
    // assert_eq!(x, 0b110);
    ($bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap | ((1 << $count) - 1) << $start_pos
        }
    };

    // let mut bitmap = 0;
    // set_bmp!(in bitmap, [start = 1, count = 2]);
    // assert_eq!(bitmap, 0b110);
    (in $bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap |= ((1 << $count) - 1) << $start_pos;
    };

    // let bitmap: u8 = 0;
    // let x = set_bmp!(bitmap, rev [1..3]);
    // assert_eq!(x, 0b_0110_0000);
    ($bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let count_to_set = $end_pos - $start_pos;
            $bitmap | (((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)))
        }
    };

    // let mut bitmap: u8 = 0;
    // set_bmp!(in bitmap, rev [1..3]);
    // assert_eq!(bitmap, 0b_0110_0000);
    (in $bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        let count_to_set = $end_pos - $start_pos;
        $bitmap |= (((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)));
    };

    // let bitmap: u8 = 0;
    // let x = set_bmp!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(x, 0b_0110_0000);
    ($bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap | (((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)))
        }
    };

    // let mut bitmap: u8 = 0;
    // set_bmp!(in bitmap, rev [start = 1, count = 2]);
    // assert_eq!(bitmap, 0b_0110_0000);
    (in $bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap |= (((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)));
    };

    // let bitmap: u8 = 0;
    // let x = set_bmp!(bitmap, rev 2);
    // assert_eq!(x, 0b_0010_0000);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap | (1 << (total_bit_count - $bit_pos - 1))
        }
    };

    // let mut bitmap: u8 = 0;
    // set_bmp!(in bitmap, rev 2);
    // assert_eq!(bitmap, 0b_0010_0000);
    (in $bitmap: ident, rev $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap |= (1 << (total_bit_count - $bit_pos - 1));
    };

    // let bitmap = 0;
    // let x = set_bmp!(bitmap, 2);
    // assert_eq!(x, 0b100);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap | (1 << $bit_pos)
        }
    };

    // let mut bitmap = 0;
    // set_bmp!(in bitmap, 2);
    // assert_eq!(bitmap, 0b100);
    (in $bitmap: ident, $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap |= 1 << $bit_pos;
    };
}

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
/// let res = unset_bmp!(bitmap, ...);
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
/// let res = unset_bmp!(0b_0000_0101, ...);
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
/// unset_bmp!(in bitmap, ...);
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
/// # use bit_fiddler::unset_bmp;
/// let mut bitmap: u8 = 0b_0010_0100;
///
/// // Unsetting 2nd bit from rhs
/// unset_bmp!(in bitmap, 2);
/// assert_eq!(bitmap, 0b_0010_0000);
///
/// // Unsetting 2nd bit from lhs
/// unset_bmp!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0b_0000_0000);
/// ```
/// # Unsetting Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for unsetting range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::unset_bmp;
///
/// // Unset third bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = unset_bmp!(bitmap, 2);
/// assert_eq!(x, 0);
///
/// // Unset third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// unset_bmp!(in bitmap, 2);
/// assert_eq!(bitmap, 0);
///
/// // Unset third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0010_0000;
/// let x = unset_bmp!(bitmap, rev 2);
/// assert_eq!(x, 0);
///
/// // Unset third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0010_0000;
/// unset_bmp!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0);
///
/// // Unset second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0b1110;
/// let x = unset_bmp!(bitmap, [1, 2, 3]);
/// assert_eq!(x, 0);
///
/// // Unset second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b1110;
/// unset_bmp!(in bitmap, [1, 2, 3]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b0111_0000;
/// let x = unset_bmp!(bitmap, rev [1, 2, 3]);
/// assert_eq!(x, 0);
///
/// // Unset second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b0111_0000;
/// unset_bmp!(in bitmap, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = unset_bmp!(bitmap, [1..3]);
/// assert_eq!(x, 0);
///
/// // Unset second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// unset_bmp!(in bitmap, [1..3]);
/// assert_eq!(bitmap, 0);
///
/// // Starting from second bit, unset 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = unset_bmp!(bitmap, [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, unset 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// unset_bmp!(in bitmap, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// // Unset second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = unset_bmp!(bitmap, rev [1..3]);
/// assert_eq!(x, 0);
///
/// // Unset second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// unset_bmp!(in bitmap, rev [1..3]);
/// assert_eq!(bitmap, 0);
/// 
/// // Starting from second bit, unset 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = unset_bmp!(bitmap, rev [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, unset 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// unset_bmp!(in bitmap, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
/// 
/// ```
#[macro_export]
macro_rules! unset_bmp { 
    // let bitmap = 0b1110;
    // let x = unset_bmp!(bitmap, [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap & !($( (1 << $bit_pos) | )* 0)
        }
    };
    // let mut bitmap = 0b1110;
    // unset_bmp!(in bitmap, [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap &= !($( (1 << $bit_pos) | )* 0);
    };

    // let bitmap: u8 = 0b0111_0000;
    // let x = unset_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap & !($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0)
        }
    };
    // let mut bitmap: u8 = 0b0111_0000;
    // unset_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap &= !($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0);
    };

    // let bitmap = 0b110;
    // let x = unset_bmp!(bitmap, [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            $bitmap & !(((1 << count_to_set) - 1) << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // unset_bmp!(in bitmap, [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let count_to_set = $end_pos - $start_pos;
        $bitmap &= !(((1 << count_to_set) - 1) << $start_pos);
    };

    // let bitmap = 0b110;
    // let x = unset_bmp!(bitmap, [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap & !(((1 << $count) - 1) << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // unset_bmp!(in bitmap, [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap &= !(((1 << $count) - 1) << $start_pos);
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = unset_bmp!(bitmap, rev [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let count_to_set = $end_pos - $start_pos;
            $bitmap & !(((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // unset_bmp!(in bitmap, rev [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        let count_to_set = $end_pos - $start_pos;
        $bitmap &= !(((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)));
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = unset_bmp!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap & !(((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // unset_bmp!(in bitmap, rev [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap &= !(((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)));
    };

    // let bitmap: u8 = 0b_0010_0000;
    // let x = unset_bmp!(bitmap, rev 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap & !(1 << (total_bit_count - $bit_pos - 1))
        }
    };

    // let mut bitmap: u8 = 0b_0010_0000;
    // unset_bmp!(in bitmap, rev 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap &= !(1 << (total_bit_count - $bit_pos - 1));
    };

    // let bitmap = 0b100;
    // let x = unset_bmp!(bitmap, 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap & !(1 << $bit_pos)
        }
    };

    // let mut bitmap = 0b100;
    // unset_bmp!(in bitmap, 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap &= !(1 << $bit_pos);
    };
}

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
/// let res = toggle_bmp!(bitmap, ...);
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
/// let res = toggle_bmp!(0b_0000_0101, ...);
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
/// toggle_bmp!(in bitmap, ...);
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
/// # use bit_fiddler::toggle_bmp;
/// let mut bitmap: u8 = 0b_0010_0100;
///
/// // Toggling 2nd and 3rd bit from rhs
/// toggle_bmp!(in bitmap, [2, 3]);
/// assert_eq!(bitmap, 0b_0010_1000);
///
/// // Unsetting 2nd and 3rd bit from lhs
/// toggle_bmp!(in bitmap, rev [2, 3]);
/// assert_eq!(bitmap, 0b_0001_1000);
/// ```
/// # Toggling Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for toggling range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::toggle_bmp;
///
/// // Toggle third bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle_bmp!(bitmap, 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle_bmp!(in bitmap, 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0010_0000;
/// let x = toggle_bmp!(bitmap, rev 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0010_0000;
/// toggle_bmp!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0b1010;
/// let x = toggle_bmp!(bitmap, [1, 2, 3]);
/// assert_eq!(x, 0b0100);
///
/// // Toggle second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b1010;
/// toggle_bmp!(in bitmap, [1, 2, 3]);
/// assert_eq!(bitmap, 0b0100);
///
/// // Toggle second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b0101_0000;
/// let x = toggle_bmp!(bitmap, rev [1, 2, 3]);
/// assert_eq!(x, 0b0010_0000);
///
/// // Toggle second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b0101_0000;
/// toggle_bmp!(in bitmap, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0b0010_0000);
///
/// // Toggle second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle_bmp!(bitmap, [1..3]);
/// assert_eq!(x, 0b010);
///
/// // Toggle second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle_bmp!(in bitmap, [1..3]);
/// assert_eq!(bitmap, 0b010);
///
/// // Starting from second bit, toggle 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = toggle_bmp!(bitmap, [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// toggle_bmp!(in bitmap, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle_bmp!(bitmap, rev [1..3]);
/// assert_eq!(x, 0);
///
/// // Toggle second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle_bmp!(in bitmap, rev [1..3]);
/// assert_eq!(bitmap, 0);
/// 
/// // Starting from second bit, toggle 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle_bmp!(bitmap, rev [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle_bmp!(in bitmap, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
/// 
/// ```
#[macro_export]
macro_rules! toggle_bmp { 
    // let bitmap = 0b1110;
    // let x = toggle_bmp!(bitmap, [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap ^ ($( (1 << $bit_pos) | )* 0)
        }
    };
    // let mut bitmap = 0b1110;
    // toggle_bmp!(in bitmap, [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap ^= $( (1 << $bit_pos) | )* 0;
    };

    // let bitmap: u8 = 0b0111_0000;
    // let x = toggle_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap ^ ($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0)
        }
    };
    // let mut bitmap: u8 = 0b0111_0000;
    // toggle_bmp!(bitmap, rev [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap ^= $( (1 << (total_bit_count - $bit_pos - 1)) | )* 0;
    };

    // let bitmap = 0b110;
    // let x = toggle_bmp!(bitmap, [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            $bitmap ^ (((1 << count_to_set) - 1) << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // toggle_bmp!(in bitmap, [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let count_to_set = $end_pos - $start_pos;
        $bitmap ^= ((1 << count_to_set) - 1) << $start_pos;
    };

    // let bitmap = 0b110;
    // let x = toggle_bmp!(bitmap, [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap ^ (((1 << $count) - 1) << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // toggle_bmp!(in bitmap, [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap ^= ((1 << $count) - 1) << $start_pos;
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = toggle_bmp!(bitmap, rev [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let count_to_set = $end_pos - $start_pos;
            $bitmap ^ (((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // toggle_bmp!(in bitmap, rev [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        let count_to_set = $end_pos - $start_pos;
        $bitmap ^= ((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1));
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = toggle_bmp!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap ^ (((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // toggle_bmp!(in bitmap, rev [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap ^= ((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1));
    };

    // let bitmap: u8 = 0b_0010_0000;
    // let x = toggle_bmp!(bitmap, rev 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            $bitmap ^ (1 << (total_bit_count - $bit_pos - 1))
        }
    };

    // let mut bitmap: u8 = 0b_0010_0000;
    // toggle_bmp!(in bitmap, rev 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
        $bitmap ^= 1 << (total_bit_count - $bit_pos - 1);
    };

    // let bitmap = 0b100;
    // let x = toggle_bmp!(bitmap, 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap ^ (1 << $bit_pos)
        }
    };

    // let mut bitmap = 0b100;
    // toggle_bmp!(in bitmap, 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap ^= 1 << $bit_pos;
    };
}

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
            $crate::check_bitmap_impl!($bitmap);
            let bits_to_check = ($( (1 << $bit_pos) | )* 0);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b0111_0000;
    // let res = is_set!(bitmap, rev [1, 2, 3]);
    // assert_eq!(res, true);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let bits_to_check = ($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap = 0b110;
    // let res = is_set!(bitmap, [1..3]);
    // assert_eq!(res, true);
    ($bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            let bits_to_check = (((1 << count_to_set) - 1) << $start_pos);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap = 0b110;
    // let res = is_set!(bitmap, [start = 1, count = 2]);
    // assert_eq!(res, true);
    ($bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let bits_to_check = (((1 << $count) - 1) << $start_pos);
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let res = is_set!(bitmap, rev [1..3]);
    // assert_eq!(res, true);
    ($bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let count_to_set = $end_pos - $start_pos;
            let bits_to_check 
                = (((1 << count_to_set) - 1) << (total_bit_count - $start_pos - 1 - (count_to_set - 1)));
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let res = is_set!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(res, true);
    ($bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            let bits_to_check
                = (((1 << $count) - 1) << (total_bit_count - $start_pos - 1 - ($count - 1)));
            ($bitmap & bits_to_check) == bits_to_check
        }
    };

    // let bitmap: u8 = 0b_0010_0000;
    // let res = is_set!(bitmap, rev 2);
    // assert_eq!(res, true);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = std::mem::size_of_val(& $bitmap ) * 8;
            ( $bitmap & (1 << (total_bit_count - $bit_pos - 1)) )
                != 0
        }
    };

    // let bitmap = 0b100;
    // let res = is_set!(bitmap, 2);
    // assert_eq!(res, true);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            ( $bitmap & (1 << $bit_pos) )
                != 0
        }
    };
}

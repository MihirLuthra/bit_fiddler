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

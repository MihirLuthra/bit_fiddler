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
/// let res = toggle!(bitmap, ...);
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
/// toggle!(in bitmap, ...);
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
/// toggle!(in bitmap, [2, 3]);
/// assert_eq!(bitmap, 0b_0010_1000);
///
/// // Unsetting 2nd and 3rd bit from lhs
/// toggle!(in bitmap, rev [2, 3]);
/// assert_eq!(bitmap, 0b_0001_1000);
/// ```
/// # Toggling Bit Ranges
/// `[<start_pos>..<end_pos>]` and `[start = <start_pos>, count = <count>]`
/// patterns are used for toggling range of bits.
/// If range has 0 bits (e.g., [3..3] or [start = 3, count = 0]), the behaviour is undefined and may panic.
///
/// # Examples
/// ```
/// use bit_fiddler::toggle;
///
/// // Toggle third bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle!(bitmap, 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle!(in bitmap, 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle third bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0010_0000;
/// let x = toggle!(bitmap, rev 2);
/// assert_eq!(x, 0);
///
/// // Toggle third bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0010_0000;
/// toggle!(in bitmap, rev 2);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second, third & fourth bit from the right and return the resulting bitmap.
/// let bitmap = 0b1010;
/// let x = toggle!(bitmap, [1, 2, 3]);
/// assert_eq!(x, 0b0100);
///
/// // Toggle second, third & fourth bit from the right in the passed bitmap itself.
/// let mut bitmap = 0b1010;
/// toggle!(in bitmap, [1, 2, 3]);
/// assert_eq!(bitmap, 0b0100);
///
/// // Toggle second, third & fourth bit from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b0101_0000;
/// let x = toggle!(bitmap, rev [1, 2, 3]);
/// assert_eq!(x, 0b0010_0000);
///
/// // Toggle second, third & fourth bit from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b0101_0000;
/// toggle!(in bitmap, rev [1, 2, 3]);
/// assert_eq!(bitmap, 0b0010_0000);
///
/// // Toggle second & third (1 and 2) bit from the right and return the resulting bitmap.
/// let bitmap = 0b100;
/// let x = toggle!(bitmap, [1..3]);
/// assert_eq!(x, 0b010);
///
/// // Toggle second & third bit (1 and 2) from the right in the passed bitmap itself.
/// let mut bitmap = 0b100;
/// toggle!(in bitmap, [1..3]);
/// assert_eq!(bitmap, 0b010);
///
/// // Starting from second bit, toggle 2 bits from the right and return the resulting bitmap.
/// let bitmap = 0b110;
/// let x = toggle!(bitmap, [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the right in the passed bitmap itself.
/// let mut bitmap = 0b110;
/// toggle!(in bitmap, [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
///
/// // Toggle second & third bit (1 and 2) from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle!(bitmap, rev [1..3]);
/// assert_eq!(x, 0);
///
/// // Toggle second & third bit (1 and 2) from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle!(in bitmap, rev [1..3]);
/// assert_eq!(bitmap, 0);
/// 
/// // Starting from second bit, toggle 2 bits from the left and return the resulting bitmap.
/// let bitmap: u8 = 0b_0110_0000;
/// let x = toggle!(bitmap, rev [start = 1, count = 2]);
/// assert_eq!(x, 0);
///
/// // Starting from second bit, toggle 2 bits from the left in the passed bitmap itself.
/// let mut bitmap: u8 = 0b_0110_0000;
/// toggle!(in bitmap, rev [start = 1, count = 2]);
/// assert_eq!(bitmap, 0);
/// 
/// ```
#[macro_export]
macro_rules! toggle { 
    // let bitmap = 0b1110;
    // let x = toggle!(bitmap, [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap ^ ($( (1 << $bit_pos) | )* 0)
        }
    };
    // let mut bitmap = 0b1110;
    // toggle!(in bitmap, [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap ^= $( (1 << $bit_pos) | )* 0;
    };

    // let bitmap: u8 = 0b0111_0000;
    // let x = toggle!(bitmap, rev [1, 2, 3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev [$( $bit_pos: tt),*]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = $crate::max_bits!($bitmap);
            $bitmap ^ ($( (1 << (total_bit_count - $bit_pos - 1)) | )* 0)
        }
    };
    // let mut bitmap: u8 = 0b0111_0000;
    // toggle!(bitmap, rev [1, 2, 3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [$( $bit_pos: tt),*]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = $crate::max_bits!($bitmap);
        $bitmap ^= $( (1 << (total_bit_count - $bit_pos - 1)) | )* 0;
    };

    // let bitmap = 0b110;
    // let x = toggle!(bitmap, [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
            $bitmap ^ (mask << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // toggle!(in bitmap, [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let count_to_set = $end_pos - $start_pos;
        let total_bit_count = $crate::max_bits!($bitmap);
        let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
        $bitmap ^= mask << $start_pos;
    };

    // let bitmap = 0b110;
    // let x = toggle!(bitmap, [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: tt, [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..$count], bit_count = total_bit_count);
            $bitmap ^ (mask << $start_pos)
        }
    };

    // let mut bitmap = 0b110;
    // toggle!(in bitmap, [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = $crate::max_bits!($bitmap);
        let mask = $crate::mask!([..$count], bit_count = total_bit_count);
        $bitmap ^= mask << $start_pos;
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = toggle!(bitmap, rev [1..3]);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev [$start_pos: tt .. $end_pos: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = $crate::max_bits!($bitmap);
            let count_to_set = $end_pos - $start_pos;
            let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
            $bitmap ^ (mask << (total_bit_count - $start_pos - 1 - (count_to_set - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // toggle!(in bitmap, rev [1..3]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [$start_pos: tt .. $end_pos: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = $crate::max_bits!($bitmap);
        let count_to_set = $end_pos - $start_pos;
        let mask = $crate::mask!([..count_to_set], bit_count = total_bit_count);
        $bitmap ^= mask << (total_bit_count - $start_pos - 1 - (count_to_set - 1));
    };

    // let bitmap: u8 = 0b_0110_0000;
    // let x = toggle!(bitmap, rev [start = 1, count = 2]);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev [start = $start_pos: tt, count = $count: tt]) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = $crate::max_bits!($bitmap);
            let mask = $crate::mask!([..$count], bit_count = total_bit_count);
            $bitmap ^ (mask << (total_bit_count - $start_pos - 1 - ($count - 1)))
        }
    };

    // let mut bitmap: u8 = 0b_0110_0000;
    // toggle!(in bitmap, rev [start = 1, count = 2]);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev [start = $start_pos: tt, count = $count: tt]) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = $crate::max_bits!($bitmap);
        let mask = $crate::mask!([..$count], bit_count = total_bit_count);
        $bitmap ^= mask << (total_bit_count - $start_pos - 1 - ($count - 1));
    };

    // let bitmap: u8 = 0b_0010_0000;
    // let x = toggle!(bitmap, rev 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, rev $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            let total_bit_count = $crate::max_bits!($bitmap);
            $bitmap ^ (1 << (total_bit_count - $bit_pos - 1))
        }
    };

    // let mut bitmap: u8 = 0b_0010_0000;
    // toggle!(in bitmap, rev 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, rev $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        let total_bit_count = $crate::max_bits!($bitmap);
        $bitmap ^= 1 << (total_bit_count - $bit_pos - 1);
    };

    // let bitmap = 0b100;
    // let x = toggle!(bitmap, 2);
    // assert_eq!(x, 0);
    ($bitmap: tt, $bit_pos: tt) => {
        {
            $crate::check_bitmap_impl!($bitmap);
            $bitmap ^ (1 << $bit_pos)
        }
    };

    // let mut bitmap = 0b100;
    // toggle!(in bitmap, 2);
    // assert_eq!(bitmap, 0);
    (in $bitmap: ident, $bit_pos: tt) => {
        $crate::check_bitmap_impl!($bitmap);
        $bitmap ^= 1 << $bit_pos;
    };
}

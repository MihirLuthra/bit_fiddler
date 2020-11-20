/// Macro for getting a bit mask over the given range.
///
/// # Example
///
/// ```
/// use bit_fiddler::mask;
///
/// let mask = mask!([0..64], u64);
/// assert_eq!(mask, u64::MAX);
///
/// let bitmap: u8 = 0b_1010_1010;
/// let masked_bitmap = bitmap & mask!([0..4], u8);
/// assert_eq!(masked_bitmap, 0b_0000_1010);
///
/// let bitmap: u8 = 0b_1111_1111;
/// let masked_bitmap = bitmap & mask!([3..], u8);
/// assert_eq!(masked_bitmap, 0b_1111_1000);
///
/// let bitmap: u8 = 0b_1111_1111;
/// let masked_bitmap = bitmap & mask!(rev [0..4], u8);
/// assert_eq!(masked_bitmap, 0b_1111_0000);
///
/// let bitmap: u8 = 0b_1111_1111;
/// let masked_bitmap = bitmap & mask!([start = 3, count = 4], u8);
/// assert_eq!(masked_bitmap, 0b_0111_1000);
/// ```
#[macro_export]
macro_rules! mask {
    ([..$end: tt], $ty: ty) => {
        {
            let max_bits = $crate::max_bits!(type = $ty);
            !(0 as $ty) >> (max_bits - $end)
        }
    };
    ([$start: tt..], $ty: ty) => {
        {
            !(0 as $ty) << $start
        }
    };
    ([..], $ty: ty) => {
        {
            !(0 as $ty)
        }
    };
    ([$start: tt..$end: tt], $ty: ty) => {
        {
            let max_bits = $crate::max_bits!(type = $ty);
            (!(0 as $ty) << $start) & (!(0 as $ty) >> (max_bits - $end))
        }
    };
    (rev [$start: tt..], $ty: ty) => {
        {
            !(0 as $ty) >> $start
        }
    };
    (rev [..$end: tt], $ty: ty) => {
        {
            let max_bits = $crate::max_bits!(type = $ty);
            !(0 as $ty) << (max_bits - $end)
        }
    };
    (rev [..], $ty: ty) => {
        {
            !(0 as $ty)
        }
    };
    (rev [$start: tt..$end: tt], $ty: ty) => {
        {
            let max_bits = $crate::max_bits!(type = $ty);
            (!(0 as $ty) << (max_bits - $end)) & (!(0 as $ty) >> $start)
        }
    };
    ([start = $start: tt, count = $count: tt], $ty: ty) => {
        {
            $crate::mask!([$start..($start + $count)], ($ty))
        }
    };
    (rev [start = $start: tt, count = $count: tt], $ty: ty) => {
        {
            $crate::mask!(rev [$start..($start + $count)], ($ty))
        }
    };
}

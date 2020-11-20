/// For getting total bit count
/// by type or identifier.
///
/// # Example
///
/// ```
/// use bit_fiddler::max_bits;
///
/// assert_eq!(max_bits!(type = i32), 32);
/// assert_eq!(max_bits!((5 as u64)), 64);
///
/// let bitmap: u8 = 0;
///
/// assert_eq!(max_bits!(bitmap), 8);
/// ```
#[macro_export]
macro_rules! max_bits {
    ($bitmap: tt) => {
        std::mem::size_of_val(&$bitmap) * 8
    };
    (type = $ty: ty) => {
        std::mem::size_of::<$ty>() * 8
    };
}

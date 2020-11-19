/// For getting total bit count
/// by type or identifier.
#[macro_export]
macro_rules! max_bits {
    ($bitmap: tt) => {
        std::mem::size_of_val(& $bitmap ) * 8
    };
    (type = $ty: ty) => {
        std::mem::size_of::<$ty>() * 8
    };
}

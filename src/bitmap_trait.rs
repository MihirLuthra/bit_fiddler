use std::ops::{
    BitOrAssign, BitOr
};

pub trait Bitmap: trait_seal::TraitSeal {
    const BIT_COUNT: usize;
}

macro_rules! impl_bitmap {
    ($ty: ty) => {
        impl Bitmap for $ty {
            const BIT_COUNT: usize = (std::mem::size_of::<$ty>() * 8);
        }
    };
}

pub fn check_bitmap_impl_by_type<T: Bitmap>() {}
pub fn check_bitmap_impl_by_value<T: Bitmap>(arg: T) {}

#[macro_export]
macro_rules! check_bitmap_impl { 
    (for type $ty: ty) => {
        // empty func calls are optimized away by the compiler
        bit_fiddler::check_bitmap_impl_by_type::<$ty>();
    };

    ($ident: ident) => {
        // empty func calls are optimized away by the compiler
        bit_fiddler::check_bitmap_impl_by_value($ident);
    };

    ($tt: tt) => {
        // empty func calls are optimized away by the compiler
        bit_fiddler::check_bitmap_impl_by_value($tt);
    };
}

impl_bitmap!(u8);
impl_bitmap!(i8);

impl_bitmap!(u16);
impl_bitmap!(i16);

impl_bitmap!(u32);
impl_bitmap!(i32);

impl_bitmap!(i64);
impl_bitmap!(u64);

impl_bitmap!(i128);
impl_bitmap!(u128);

mod trait_seal {
    pub trait TraitSeal {}
    macro_rules! impl_trait_seal {
        ($ty: ty) => {
            impl TraitSeal for $ty {}
        };
    }

	impl_trait_seal!(u8);
	impl_trait_seal!(i8);
	
	impl_trait_seal!(u16);
	impl_trait_seal!(i16);
	
	impl_trait_seal!(u32);
	impl_trait_seal!(i32);
	
	impl_trait_seal!(i64);
	impl_trait_seal!(u64);
	
	impl_trait_seal!(i128);
	impl_trait_seal!(u128);
}

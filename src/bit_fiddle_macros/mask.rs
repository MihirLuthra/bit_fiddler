/// This will change in subsequent versions for 
/// supporting multiple patterns. Currently it's just a dependency
/// of other macros. 
#[doc(hidden)]
#[macro_export]
macro_rules! mask {
    ([..$count: tt], bit_count = $bit_count: tt) => {
        {
            $crate::mask!([0..$count], bit_count = $bit_count)
        }
    };
    ([$start: tt..], bit_count = $bit_count: tt) => {
        {
            !0 << $start
        }
    };
    ([..], bit_count = $bit_count: tt) => {
        {
            !0
        }
    };
    ([$start: tt..$end: tt], bit_count = $bit_count: tt) => {
        {
            let dec_bit_count = $bit_count - 1;
            [(1 << ($end & dec_bit_count)) - 1, !0][((($end & !dec_bit_count)) != 0) as usize] << $start
        }
    };
}

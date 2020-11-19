#[macro_export]
macro_rules! mask {
    ([..$count: tt], bit_count = $bit_count: tt) => {
        {
            let dec_bit_count = $bit_count - 1;
            [(1 << ($count & dec_bit_count)) - 1, !0][((($count & !dec_bit_count)) != 0) as usize]
        }
    };/*
    ($bitmap: tt, [..$count: tt]) => {
        {
            let bit_count = std::mem::size_of_val(& $bitmap) * 8;
            let dec_bit_count = bit_count - 1;
            $bitmap & [(1 << ($count & dec_bit_count)) - 1, !1][((($count & !dec_bit_count)) != 0) as usize]
        }
    };*/
}

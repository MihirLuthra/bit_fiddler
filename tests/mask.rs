use bit_fiddler::{mask, max_bits};

#[test]
fn range_with_bit_count() {
    let bitmap: u8 = 0b_1111_1111;
    let max_bits = max_bits!(bitmap);

    let masked_bitmap = bitmap & mask!([0..5], bit_count = max_bits);
    assert_eq!(masked_bitmap, 0b_0001_1111);

    let masked_bitmap = bitmap & mask!([0..8], bit_count = max_bits);
    assert_eq!(masked_bitmap, 0b_1111_1111);

    let masked_bitmap = bitmap & mask!([3..], bit_count = max_bits);
    assert_eq!(masked_bitmap, 0b_1111_1000);

    let masked_bitmap = bitmap & mask!([..4], bit_count = max_bits);
    assert_eq!(masked_bitmap, 0b_0000_1111);

    let masked_bitmap = bitmap & mask!([..], bit_count = max_bits);
    assert_eq!(masked_bitmap, 0b_1111_1111);
}

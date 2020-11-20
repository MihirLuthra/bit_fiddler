use bit_fiddler::mask;

#[test]
fn range_with_bit_count() {
    let bitmap = 0b_1111_1111;

    let masked_bitmap = bitmap & mask!([0..5], u8);
    assert_eq!(masked_bitmap, 0b_0001_1111);

    let masked_bitmap = bitmap & mask!([0..8], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);

    let masked_bitmap = bitmap & mask!([2..8], u8);
    assert_eq!(masked_bitmap, 0b_1111_1100);

    let masked_bitmap = bitmap & mask!([3..7], u8);
    assert_eq!(masked_bitmap, 0b_0111_1000);

    let masked_bitmap = bitmap & mask!([3..], u8);
    assert_eq!(masked_bitmap, 0b_1111_1000);

    let masked_bitmap = bitmap & mask!([..5], u8);
    assert_eq!(masked_bitmap, 0b_0001_1111);

    let masked_bitmap = bitmap & mask!([..], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);
}

#[test]
fn range_with_bit_count_rev() {
    let bitmap = 0b_1111_1111;

    let masked_bitmap = bitmap & mask!(rev [0..5], u8);
    assert_eq!(masked_bitmap, 0b_1111_1000);

    let masked_bitmap = bitmap & mask!(rev [0..8], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);

    let masked_bitmap = bitmap & mask!(rev [2..8], u8);
    assert_eq!(masked_bitmap, 0b_0011_1111);

    let masked_bitmap = bitmap & mask!(rev [0..7], u8);
    assert_eq!(masked_bitmap, 0b_1111_1110);

    let masked_bitmap = bitmap & mask!(rev [2..7], u8);
    assert_eq!(masked_bitmap, 0b_0011_1110);

    let masked_bitmap = bitmap & mask!(rev [3..], u8);
    assert_eq!(masked_bitmap, 0b_0001_1111);

    let masked_bitmap = bitmap & mask!(rev [..4], u8);
    assert_eq!(masked_bitmap, 0b_1111_0000);

    let masked_bitmap = bitmap & mask!(rev [..], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);
}

#[test]
fn counted_range_with_bit_count() {
    let bitmap = 0b_1111_1111;

    let masked_bitmap = bitmap & mask!([start = 0, count = 5], u8);
    assert_eq!(masked_bitmap, 0b_0001_1111);

    let masked_bitmap = bitmap & mask!([start = 0, count = 8], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);
}

#[test]
fn counted_range_with_bit_count_rev() {
    let bitmap = 0b_1111_1111;

    let masked_bitmap = bitmap & mask!(rev [start = 0, count = 5], u8);
    assert_eq!(masked_bitmap, 0b_1111_1000);

    let masked_bitmap = bitmap & mask!(rev [start = 0, count = 8], u8);
    assert_eq!(masked_bitmap, 0b_1111_1111);
}

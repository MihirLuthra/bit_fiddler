use bit_fiddler::unset;

#[test]
fn unset_single_bit() {
    let mut bitmap: u8 = 0b_1000_0000;
    
    let res = unset!(bitmap, 7);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1000_0000);

    unset!(in bitmap, rev 0);
    assert_eq!(bitmap, 0);
}

#[test]
fn unset_multiple_bits() {
    let mut bitmap: u8 = 0b_0011_1000;

    let res = unset!(bitmap, [3, 4, 5]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0011_1000);

    unset!(in bitmap, rev [3, 4, 5]);
    assert_eq!(bitmap, 0b_0010_0000);
}

#[test]
fn unset_range() {
    let mut bitmap: u8 = 0b_1111_1111;

    let res = unset!(bitmap, [0..8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, rev [1..7]);
    assert_eq!(bitmap, 0b_1000_0001);
}

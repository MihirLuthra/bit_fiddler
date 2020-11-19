use bit_fiddler::is_set;

#[test]
fn is_set_single_bit() {
    let bitmap: u8 = 0b_1000_0000;
    
    let res = is_set!(bitmap, 7);
    assert_eq!(res, true);

    let res = is_set!(bitmap, 6);
    assert_eq!(res, false);
}

#[test]
fn is_set_single_bit_rev() {
    let bitmap: u8 = 0b_1000_0000;
    
    let res = is_set!(bitmap, rev 0);
    assert_eq!(res, true);

    let res = is_set!(bitmap, rev 3);
    assert_eq!(res, false);
}

#[test]
fn is_set_multiple_bits() {
    let bitmap: u8 = 0b_0011_1000;

    let res = is_set!(bitmap, [3, 4, 5]);
    assert_eq!(res, true);

    let res = is_set!((0b_0101_0101 as u8), [1, 5]);
    assert_eq!(res, false);
}

#[test]
fn is_set_multiple_bits_rev() {
    let res = is_set!((0b_0101_0101 as u8), rev [1, 5]);
    assert_eq!(res, true);

    let res = is_set!((0b_0101_0101 as u8), rev [0, 7]);
    assert_eq!(res, false);
}

#[test]
fn is_set_range() {
    let bitmap: u8 = 0b_1111_1111;

    let res = is_set!(bitmap, [0..8]);
    assert_eq!(res, true);

    let res = is_set!(0b_0110_1100, [2..4]);
    assert_eq!(res, true);
}

#[test]
fn is_set_range_rev() {
    let bitmap: u8 = 0b_1111_1111;

    let res = is_set!(bitmap, rev [0..8]);
    assert_eq!(res, true);

    let res = is_set!(0b_0110_1100, rev [2..4]);
    assert_eq!(res, false);
}

#[test]
fn is_set_counted_range() {
    let bitmap: u8 = 0b_1111_1111;

    let res = is_set!(bitmap, [start = 0, count = 8]);
    assert_eq!(res, true);

    let res = is_set!(0b_0110_1100, [start = 2, count = 2]);
    assert_eq!(res, true);
}

#[test]
fn is_set_counted_range_rev() {
    let bitmap: u8 = 0b_1111_1111;

    let res = is_set!(bitmap, rev [start = 0, count = 8]);
    assert_eq!(res, true);

    let res = is_set!(0b_0110_1100, rev [start = 1, count = 5]);
    assert_eq!(res, false);
}

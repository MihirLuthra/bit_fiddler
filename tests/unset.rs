use bit_fiddler::unset;

#[test]
fn unset_single_bit() {
    let mut bitmap: u8 = 0b_1000_0001;

    let res = unset!(bitmap, u8, 7);
    assert_eq!(res, 0b1);
    assert_eq!(bitmap, 0b_1000_0001);

    unset!(in bitmap, u8, 0);
    assert_eq!(bitmap, 0b_1000_0000);
}

#[test]
fn unset_single_bit_rev() {
    let mut bitmap: u8 = 0b_1000_0001;

    let res = unset!(bitmap, u8, rev 7);
    assert_eq!(res, 0b_1000_0000);
    assert_eq!(bitmap, 0b_1000_0001);

    unset!(in bitmap, u8, rev 0);
    assert_eq!(bitmap, 0b_0000_0001);
}

#[test]
fn unset_multiple_bits() {
    let mut bitmap: u8 = 0b_0000_1001;

    let res = unset!(bitmap, u8, [3, 0, 5]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0000_1001);

    unset!(in bitmap, u8, [1, 7, 5, 3]);
    assert_eq!(bitmap, 0b_0000_0001);
}

#[test]
fn unset_multiple_bits_rev() {
    let mut bitmap: u8 = 0b_0100_1001;

    let res = unset!(bitmap, u8, rev [1, 7, 4]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0100_1001);

    bitmap = 0b_1100_0100;

    unset!(in bitmap, u8, rev [5, 1, 0]);
    assert_eq!(bitmap, 0);
}

#[test]
fn unset_range() {
    let mut bitmap: u8 = 0b_1111_1111;

    let res = unset!(bitmap, u8, [0..8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1111_1111);

    bitmap = 0b_1111_1111;

    unset!(in bitmap, u8, [2..6]);
    assert_eq!(bitmap, 0b_1100_0011);

    let mut bitmap: u8 = 0b_1111_1111;

    let res = unset!(bitmap, u8, [3..]);
    assert_eq!(res, 0b_0000_0111);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, [0..]);
    assert_eq!(bitmap, 0);

    bitmap = 0b_1111_1111;

    let res = unset!(bitmap, u8, [..5]);
    assert_eq!(res, 0b_1110_0000);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, [..8]);
    assert_eq!(bitmap, 0);

    bitmap = 0b_1111_1111;

    let res = unset!(bitmap, u8, [..]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, [..]);
    assert_eq!(bitmap, 0);
}

#[test]
fn unset_range_rev() {
    let mut bitmap: u8 = 0b_0111_1111;

    let res = unset!(bitmap, u8, rev[1..8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0111_1111);

    bitmap = 0b_1000_0000;

    unset!(in bitmap, u8, rev [0..1]);
    assert_eq!(bitmap, 0);

    let mut bitmap: u8 = 0b_1111_1111;

    let res = unset!(bitmap, u8, rev [3..]);
    assert_eq!(res, 0b_1110_0000);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, rev [0..]);
    assert_eq!(bitmap, 0);

    bitmap = 0b_1111_1111;

    let res = unset!(bitmap, u8, rev [..5]);
    assert_eq!(res, 0b_0000_0111);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, rev [..8]);
    assert_eq!(bitmap, 0);

    bitmap = 0b_1111_1111;

    let res = unset!(bitmap, u8, rev [..]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1111_1111);

    unset!(in bitmap, u8, rev [..]);
    assert_eq!(bitmap, 0);
}

#[test]
fn unset_counted_range() {
    let mut bitmap: u8 = 0b_1000_0000;

    let res = unset!(bitmap, u8, [start = 0, count = 8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1000_0000);

    bitmap = 0b_0110_0000;

    unset!(in bitmap, u8, [start = 5, count = 2]);
    assert_eq!(bitmap, 0);
}

#[test]
fn unset_counted_range_rev() {
    let mut bitmap: u8 = 0b_0110_0000;

    let res = unset!(bitmap, u8, rev [start = 1, count = 2]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0110_0000);

    bitmap = 0b_0111_1100;

    unset!(in bitmap, u8, rev [start = 1, count = 5]);
    assert_eq!(bitmap, 0);
}

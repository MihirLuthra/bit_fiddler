use bit_fiddler::set;

#[test]
fn set_single_bit() {
    let mut bitmap: u8 = 0;
    
    let res = set!(bitmap, 7);
    assert_eq!(res, 0b_1000_0000);
    assert_eq!(bitmap, 0);

    set!(in bitmap, rev 0);
    assert_eq!(bitmap, 0b_1000_0000);
}

#[test]
fn set_multiple_bits() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, [3, 4, 5]);
    assert_eq!(res, 0b_0011_1000);
    assert_eq!(bitmap, 0);

    set!(in bitmap, rev [3, 4, 5]);
    assert_eq!(bitmap, 0b_0001_1100);
}

#[test]
fn set_range() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, [0..8]);
    assert_eq!(res, 0b_1111_1111);
    assert_eq!(bitmap, 0);
    
    set!(in bitmap, rev [3..7]);
    assert_eq!(bitmap, 0b_0001_1110);
}

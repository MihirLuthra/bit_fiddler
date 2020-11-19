use bit_fiddler::set;

#[test]
fn set_single_bit() {
    let mut bitmap: u8 = 0;
    
    let res = set!(bitmap, 7);
    assert_eq!(res, 0b_1000_0000);
    assert_eq!(bitmap, 0);

    set!(in bitmap, 0);
    assert_eq!(bitmap, 0b_0000_0001);
}

#[test]
fn set_single_bit_rev() {
    let mut bitmap: u8 = 0;
    
    let res = set!(bitmap, rev 7);
    assert_eq!(res, 0b_0000_0001);
    assert_eq!(bitmap, 0);

    set!(in bitmap, rev 0);
    assert_eq!(bitmap, 0b_1000_0000);
}

#[test]
fn set_multiple_bits() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, [3, 0, 5]);
    assert_eq!(res, 0b_0010_1001);
    assert_eq!(bitmap, 0);

    set!(in bitmap, [1, 7, 5]);
    assert_eq!(bitmap, 0b_1010_0010);
}

#[test]
fn set_multiple_bits_rev() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, rev [1, 7, 4]);
    assert_eq!(res, 0b_0100_1001);
    assert_eq!(bitmap, 0);

    set!(in bitmap, rev [5, 1, 0]);
    assert_eq!(bitmap, 0b_1100_0100);
}

#[test]
fn set_range() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, [0..8]);
    assert_eq!(res, 0b_1111_1111);
    assert_eq!(bitmap, 0);
    
    set!(in bitmap, [1..5]);
    assert_eq!(bitmap, 0b_0001_1110);
}

#[test]
fn set_range_rev() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, rev [1..8]);
    assert_eq!(res, 0b_0111_1111);
    assert_eq!(bitmap, 0);
    
    set!(in bitmap, rev [0..1]);
    assert_eq!(bitmap, 0b_1000_0000);
}

#[test]
fn set_counted_range() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, [start = 0, count = 8]);
    assert_eq!(res, 0b_1111_1111);
    assert_eq!(bitmap, 0);
    
    set!(in bitmap, [start = 5, count = 2]);
    assert_eq!(bitmap, 0b_0110_0000);
}

#[test]
fn set_counted_range_rev() {
    let mut bitmap: u8 = 0;

    let res = set!(bitmap, rev [start = 6, count = 1]);
    assert_eq!(res, 0b_0000_0010);
    assert_eq!(bitmap, 0);
    
    set!(in bitmap, rev [start = 1, count = 5]);
    assert_eq!(bitmap, 0b_0111_1100);
}

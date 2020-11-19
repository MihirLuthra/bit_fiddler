use bit_fiddler::toggle;

#[test]
fn toggle_single_bit() {
    let mut bitmap: u8 = 0b_1000_0000;
    
    let res = toggle!(bitmap, 7);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1000_0000);

    toggle!(in bitmap, rev 0);
    assert_eq!(bitmap, 0);
}

#[test]
fn toggle_multiple_bits() {
    let mut bitmap: u8 = 0b_0011_1000;

    let res = toggle!(bitmap, [3, 4, 5]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0011_1000);

    toggle!(in bitmap, rev [3, 4, 5]);
    assert_eq!(bitmap, 0b_0010_0100);
}

#[test]
fn toggle_range() {
    let mut bitmap: u8 = 0b_0011_1111;

    let res = toggle!(bitmap, [0..8]);
    assert_eq!(res, 0b_1100_0000);
    assert_eq!(bitmap, 0b_0011_1111);

    toggle!(in bitmap, rev [0..6]);
    assert_eq!(bitmap, 0b_1100_0011);
}

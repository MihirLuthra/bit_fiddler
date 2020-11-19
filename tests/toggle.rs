use bit_fiddler::toggle;

#[test]
fn toggle_single_bit() {
    let mut bitmap: u8 = 0b_1000_0001;
    
    let res = toggle!(bitmap, 7);
    assert_eq!(res, 0b1);
    assert_eq!(bitmap, 0b_1000_0001);

    toggle!(in bitmap, 0);
    assert_eq!(bitmap, 0b_1000_0000);
}

#[test]
fn toggle_single_bit_rev() {
    let mut bitmap: u8 = 0b_1000_0001;
    
    let res = toggle!(bitmap, rev 7);
    assert_eq!(res, 0b_1000_0000);
    assert_eq!(bitmap, 0b_1000_0001);

    toggle!(in bitmap, rev 0);
    assert_eq!(bitmap, 0b_0000_0001);
}

#[test]
fn toggle_multiple_bits() {
    let mut bitmap: u8 = 0b_0000_1001;

    let res = toggle!(bitmap, [3, 0, 5]);
    assert_eq!(res, 0b_0010_0000);
    assert_eq!(bitmap, 0b_0000_1001);

    toggle!(in bitmap, [1, 7, 5, 3]);
    assert_eq!(bitmap, 0b_1010_0011);
}

#[test]
fn toggle_multiple_bits_rev() {
    let mut bitmap: u8 = 0b_0100_1001;

    let res = toggle!(bitmap, rev [1, 7, 4]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0100_1001);

    bitmap = 0b_1100_0100;

    toggle!(in bitmap, rev [5, 1, 0]);
    assert_eq!(bitmap, 0);
}

#[test]
fn toggle_range() {
    let mut bitmap: u8 = 0b_1111_1111;

    let res = toggle!(bitmap, [0..8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_1111_1111);

    bitmap = 0b_1111_1111;
    
    toggle!(in bitmap, [2..6]);
    assert_eq!(bitmap, 0b_1100_0011);
}

#[test]
fn toggle_range_rev() {
    let mut bitmap: u8 = 0b_0111_1111;

    let res = toggle!(bitmap, rev [1..8]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0111_1111);

    bitmap = 0b_1000_0000;
    
    toggle!(in bitmap, rev [0..1]);
    assert_eq!(bitmap, 0);
}

#[test]
fn toggle_counted_range() {
    let mut bitmap: u8 = 0b_1000_0000;

    let res = toggle!(bitmap, [start = 0, count = 8]);
    assert_eq!(res, 0b_0111_1111);
    assert_eq!(bitmap, 0b_1000_0000);

    bitmap = 0b_0110_0000;
    
    toggle!(in bitmap, [start = 5, count = 2]);
    assert_eq!(bitmap, 0);
}

#[test]
fn toggle_counted_range_rev() {
    let mut bitmap: u8 = 0b_0110_0000;

    let res = toggle!(bitmap, rev [start = 1, count = 2]);
    assert_eq!(res, 0);
    assert_eq!(bitmap, 0b_0110_0000);
    
    bitmap = 0b_0111_1100;

    toggle!(in bitmap, rev [start = 1, count = 5]);
    assert_eq!(bitmap, 0);
}

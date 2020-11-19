use bit_fiddler::max_bits;

#[test]
fn max_bits_for_ident() {
    let bitmap: u128 = 9;
    let max_bits = max_bits!(bitmap);
    
    assert_eq!(max_bits, 128);
}

#[test]
fn max_bits_for_literal() {
    let max_bits = max_bits!((9 as u8));
    assert_eq!(max_bits, 8);
}

#[test]
fn max_bits_for_type() {
    let max_bits = max_bits!(type = u64);
    assert_eq!(max_bits, 64);
}

use oid_str::{ARC_LEN, Arc, write_b128};

#[test]
fn test_encode_small() {
    let mut buffer = [0; ARC_LEN];
    let oid = write_b128(&mut buffer, 1);
    assert_eq!(oid.as_bytes(), b"\x01");
}

#[test]
fn test_encode_zero() {
    let mut buffer = [0; ARC_LEN];
    let oid = write_b128(&mut buffer, 0);
    assert_eq!(oid.as_bytes(), b"\x00");
}

#[test]
fn test_encode_large() {
    let mut buffer = [0; ARC_LEN];
    let oid = write_b128(&mut buffer, 257);
    assert_eq!(oid.as_bytes(), b"\x82\x01");
}

#[test]
fn test_encode_large_with_zeros() {
    let mut buffer = [0; ARC_LEN];
    let oid = write_b128(&mut buffer, 128 * 128);
    assert_eq!(oid.as_bytes(), b"\x81\x80\x00");
}

#[test]
fn test_encode_maximal_size() {
    let mut buffer = [0; ARC_LEN];
    assert_eq!(Arc::MAX, 0xffff_ffff);
    let oid = write_b128(&mut buffer, 0xffff_ffff);
    assert_eq!(oid.as_bytes(), b"\x8f\xff\xff\xff\x7f");

    let oid = write_b128(&mut buffer, 0xffff_fffe);
    assert_eq!(oid.as_bytes(), b"\x8f\xff\xff\xff\x7e");
}
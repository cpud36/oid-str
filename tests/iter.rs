use oid_str::{AbsoluteOid, Arc, RelativeOid};

#[test]
fn iter_arcs_maximal_size() {
    assert_eq!(Arc::MAX, 0xffff_ffff);
    let oid = RelativeOid::from_bytes(b"\x8f\xff\xff\xff\x7f").unwrap();
    let arcs: Vec<_> = oid.arcs().collect();
    assert_eq!(arcs, &[0xffff_ffff]);
}

#[test]
fn iter_absolute_arcs0() {
    let oid = AbsoluteOid::from_bytes(b"\x02\x86\x48\xCE\x3D\x02\x01").unwrap();
    let arcs: Vec<_> = oid.arcs().collect();
    assert_eq!(arcs, [0, 2, 840, 10045, 2, 1]);
}

#[test]
fn iter_absolute_arcs1() {
    let oid = AbsoluteOid::from_bytes(b"\x2A\x86\x48\xCE\x3D\x02\x01").unwrap();
    let arcs: Vec<_> = oid.arcs().collect();
    assert_eq!(arcs, [1, 2, 840, 10045, 2, 1]);
}

#[test]
fn iter_absolute_arcs2() {
    let oid = AbsoluteOid::from_bytes(b"\x60\x86\x48\x01\x65\x03\x04\x01\x2A").unwrap();
    let arcs: Vec<_> = oid.arcs().collect();
    assert_eq!(arcs, [2, 16, 840, 1, 101, 3, 4, 1, 42]);
}

#[test]
fn iter_large_with_zeros() {
    let oid = RelativeOid::from_bytes(b"\x81\x80\x00").unwrap();
    let arcs: Vec<_> = oid.arcs().collect();
    assert_eq!(arcs, &[128 * 128]);
}
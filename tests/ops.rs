use oid_str::{AbsoluteOid, RelativeOid};

#[test]
fn split_at() {
    let oid = RelativeOid::from_bytes(b"\x01\x02\x82\x01").unwrap();
    assert_eq!(oid.to_string(), ".1.2.257");

    let (first, last) = oid.split_at(2);

    assert_eq!(first.to_string(), ".1.2");
    assert_eq!(last.to_string(), ".257");
}

#[test]
fn split_at_start() {
    let oid = RelativeOid::from_bytes(b"\x01\x02\x03").unwrap();
    assert_eq!(oid.to_string(), ".1.2.3");

    let (first, last) = oid.split_at(0);

    assert_eq!(first.to_string(), "");
    assert_eq!(last.to_string(), ".1.2.3");
}

#[test]
fn split_at_end() {
    let oid = RelativeOid::from_bytes(b"\x01\x02\x03").unwrap();
    assert_eq!(oid.to_string(), ".1.2.3");

    let (first, last) = oid.split_at(3);

    assert_eq!(first.to_string(), ".1.2.3");
    assert_eq!(last.to_string(), "");
}

#[test]
fn starts_with() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let prefix = RelativeOid::from_bytes(b"\x01\x82\x03").unwrap();
    assert_eq!(prefix.to_string(), ".1.259");

    assert!(oid.starts_with(prefix))
}

#[test]
fn not_starts_with() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let prefix = RelativeOid::from_bytes(b"\x01\x02").unwrap();
    assert_eq!(prefix.to_string(), ".1.2");

    assert!(!oid.starts_with(prefix))
}

#[test]
fn ends_with() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let suffix = RelativeOid::from_bytes(b"\x82\x03\x04").unwrap();
    assert_eq!(suffix.to_string(), ".259.4");

    assert!(oid.ends_with(suffix))
}

#[test]
fn ends_with_in_the_middle() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let suffix = RelativeOid::from_bytes(b"\x03\x04").unwrap();
    assert_eq!(suffix.to_string(), ".3.4");

    assert!(!oid.ends_with(suffix))
}

#[test]
fn strip_prefix() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x81\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.132");

    let prefix = RelativeOid::from_bytes(b"\x01\x82\x03").unwrap();
    assert_eq!(prefix.to_string(), ".1.259");

    let suffix = oid.strip_prefix(prefix).unwrap();
    assert_eq!(suffix.to_string(), ".132")
}

#[test]
fn strip_prefix_long() {
    let oid = RelativeOid::from_bytes(b"\x01\x82\x03\x81\x04").unwrap();
    assert_eq!(oid.to_string(), ".1.259.132");

    let prefix = RelativeOid::from_bytes(b"\x01\x82\x03\x81\x04\x01").unwrap();
    assert_eq!(prefix.to_string(), ".1.259.132.1");

    let suffix = oid.strip_prefix(prefix);
    assert!(suffix.is_none());
}

#[test]
fn strip_suffix() {
    let oid = AbsoluteOid::from_bytes(b"\x10\x01\x82\x03\x81\x04").unwrap();
    assert_eq!(oid.to_string(), "0.16.1.259.132");

    let suffix = RelativeOid::from_bytes(b"\x81\x04").unwrap();
    assert_eq!(suffix.to_string(), ".132");

    let prefix = oid.strip_suffix(suffix).unwrap();
    assert_eq!(prefix.to_string(), "0.16.1.259")
}

#[test]
fn copy_from() {
    let mut buffer = *b"\x01\x82\x03\x04";
    let oid = RelativeOid::from_mut_bytes(&mut buffer).unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let part = RelativeOid::from_bytes(b"\x81\x00").unwrap();
    assert_eq!(part.to_string(), ".128");

    oid[1..3].copy_from(part);

    assert_eq!(oid.to_string(), ".1.128.4");
}

#[test]
fn copy_changes_n_arcs() {
    let mut buffer = *b"\x01\x82\x03\x04";
    let oid = RelativeOid::from_mut_bytes(&mut buffer).unwrap();
    assert_eq!(oid.to_string(), ".1.259.4");

    let part = RelativeOid::from_bytes(b"\x02\x00").unwrap();
    assert_eq!(part.to_string(), ".2.0");

    oid[1..3].copy_from(part);

    assert_eq!(oid.to_string(), ".1.2.0.4");
}

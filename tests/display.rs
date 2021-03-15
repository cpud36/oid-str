use oid_str::AbsoluteOidVec;

#[test]
fn test_display0() {
    let oid: AbsoluteOidVec = "0.2.840.10045.2.1".parse().unwrap();
    let arcs = oid.to_string();
    assert_eq!(arcs, "0.2.840.10045.2.1");
}

#[test]
fn test_display1() {
    let oid: AbsoluteOidVec = "1.2.840.10045.2.1".parse().unwrap();
    let arcs = oid.to_string();
    assert_eq!(arcs, "1.2.840.10045.2.1");
}

#[test]
fn test_display2() {
    let oid: AbsoluteOidVec = "2.16.840.1.101.3.4.1.42".parse().unwrap();
    let arcs = oid.to_string();
    assert_eq!(arcs, "2.16.840.1.101.3.4.1.42");
}

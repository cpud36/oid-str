use oid_str::{AbsoluteOid, AbsoluteOidVec, Arc, OidParsingError, RelativeOidVec};


#[test]
fn parse_absolute_0() {
    let oid: AbsoluteOidVec = "0.2.840.10045.2.1".parse().unwrap();
    let expected = AbsoluteOid::from_bytes(b"\x02\x86\x48\xCE\x3D\x02\x01").unwrap();
    assert_eq!(oid.to_string(), "0.2.840.10045.2.1");
    assert_eq!(oid.as_bytes(), expected.as_bytes());
}

#[test]
fn parse_absolute_1() {
    let oid: AbsoluteOidVec = "1.2.840.10045.2.1".parse().unwrap();
    let expected = AbsoluteOid::from_bytes(b"\x2A\x86\x48\xCE\x3D\x02\x01").unwrap();
    assert_eq!(oid.to_string(), "1.2.840.10045.2.1");
    assert_eq!(oid.as_bytes(), expected.as_bytes());
}

#[test]
fn parse_absolute_2() {
    let oid: AbsoluteOidVec = "2.16.840.1.101.3.4.1.42".parse().unwrap();
    let expected = AbsoluteOid::from_bytes(b"\x60\x86\x48\x01\x65\x03\x04\x01\x2A").unwrap();
    assert_eq!(oid.to_string(), "2.16.840.1.101.3.4.1.42");
    assert_eq!(oid.as_bytes(), expected.as_bytes());
}

#[test]
fn parse_absolute_with_leading_dot() {
    let oid: AbsoluteOidVec = ".1.3.6.1.4".parse().unwrap();
    assert_eq!(oid.to_string(), "1.3.6.1.4");
    assert_eq!(oid.as_bytes(), b"\x2b\x06\x01\x04");
}

#[test]
fn parse_absolute_failure_double_dot() {
    let result = ".1.3..1.4".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(4)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_invalid_char() {
    let result = ".1.3.1d.4".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::InvalidChar(6)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_unfinished_last_segment() {
    let result = ".1.3.1.4.".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(8)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_empty() {
    let result = "".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(0)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_no_arc1() {
    let result = ".1".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::NoArc1),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_no_arc1_with_dot() {
    let result = ".1.".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(2)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_no_arc0_overflow() {
    let result = ".3.6".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::OverflowError),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_failure_arc1_overflow() {
    let result = ".1.40".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::OverflowError),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_absolute_arc1_max_value() {
    let oid: AbsoluteOidVec = ".1.39".parse().unwrap();
    assert_eq!(oid.as_bytes(), b"\x4f");
}

#[test]
fn parse_absolute_failure_only_dot() {
    let result = ".".parse::<AbsoluteOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(0)),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_relative_empty() {
    let result: RelativeOidVec = "".parse().unwrap();
    assert_eq!(result.as_bytes(), b"");
}

#[test]
fn parse_relative_empty_dot() {
    let result: RelativeOidVec = ".".parse().unwrap();
    assert_eq!(result.as_bytes(), b"");
}

#[test]
fn parse_relative() {
    let oid: RelativeOidVec = "840.10045.2.1".parse().unwrap();
    assert_eq!(oid.as_bytes(), b"\x86\x48\xCE\x3D\x02\x01");
}

#[test]
fn parse_relative_max_value() {
    assert_eq!(Arc::MAX, 4294967295);
    let oid: RelativeOidVec = "1.4294967295".parse().unwrap();
    assert_eq!(oid.as_bytes(), b"\x01\x8f\xff\xff\xff\x7f")
}

#[test]
fn parse_relative_arc_overflow() {
    assert_eq!(Arc::MAX, 4294967295);
    let result = "1.4294967296".parse::<RelativeOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::OverflowError),
        "result is {:?}",
        result
    );
}

#[test]
fn parse_relative_double_dot() {
    let result = "1.3.4..8.1.2".parse::<RelativeOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::IntegerExpected(5)),
        "result is {:?}",
        result
    );
}


#[test]
fn parse_relative_letter() {
    let result = "1.3.4.f.8.1.2".parse::<RelativeOidVec>().unwrap_err();
    assert!(
        matches!(result, OidParsingError::InvalidChar(6)),
        "result is {:?}",
        result
    );
}
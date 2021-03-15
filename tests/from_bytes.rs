use oid_str::{AbsoluteOid, B128ErrorKind, OidDecodingError, RelativeOid, RootOid};

#[test]
fn root_cannot_be_build_from_too_large_bytes_120() {
    let result = RootOid::from_u8(120u8);
    assert!(result.is_err(), "root cannot be built from byte 120");
}

#[test]
fn root_cannot_be_build_from_too_large_bytes_255() {
    let result = RootOid::from_u8(255u8);
    assert!(result.is_err(), "root cannot be built from byte 255");
}

#[test]
fn relative_oid_b128_error_overflow() {
    let bytes = b"\x90\x80\x80\x80\x00";
    let result = RelativeOid::from_bytes(bytes);
    assert!(result.is_err(), "segments of 33 bits length are not allowed (Segment type = u32): result={:?}", result.unwrap());
    let err = result.unwrap_err();
    assert!(matches!(err.kind, B128ErrorKind::OutOfRange), "out of range error expected: kind={:?}", err.kind);
    assert_eq!(err.pos, 0);
}

#[test]
fn relative_oid_b128_error_zero_continuation() {
    let bytes = b"\x80";
    let result = RelativeOid::from_bytes(bytes);
    assert!(result.is_err(), "empty first continuation is not allowed");
    let err = result.unwrap_err();
    assert!(matches!(err.kind, B128ErrorKind::ZeroByteWithCont), "zero continuation error expected: kind={:?}", err.kind);
    assert_eq!(err.pos, 0);
}

#[test]
fn relative_oid_b128_error_unfinished_continuation() {
    let bytes = b"\x81\x80";
    let result = RelativeOid::from_bytes(bytes);
    assert!(result.is_err(), "unfinished continuation is not allowed");
    let err = result.unwrap_err();
    assert!(matches!(err.kind, B128ErrorKind::Unfinished), "unfinished continuation error expected: kind={:?}", err.kind);
    assert_eq!(err.pos, 0);
}

#[test]
fn absolute_oid_empty() {
    let bytes = b"";
    let result = AbsoluteOid::from_bytes(bytes);
    assert!(result.is_err(), "absolute oid must contain root element");
    let err = result.unwrap_err();
    assert!(matches!(err, OidDecodingError::Empty), "error is {:?}", err);
}

#[test]
fn absolute_oid_root_overflow() {
    let bytes = b"\x78";
    let result = AbsoluteOid::from_bytes(bytes);
    assert!(result.is_err(), "absolute oid root must be valid");
    let err = result.unwrap_err();
    assert!(matches!(err, OidDecodingError::Root(_)), "error is {:?}", err);
}

#[test]
fn absolute_oid_b128_overflow() {
    let mut bytes = *b"\x43\x90\x80\x80\x80\x80";
    let result = AbsoluteOid::from_mut_bytes(&mut bytes);
    assert!(result.is_err(), "absolute oid must contain valid tail");
    let err = result.unwrap_err();
    assert!(matches!(err, OidDecodingError::Base128(_)), "error is {:?}", err);
}

#[test]
fn absolute_oid_from_mut_bytes() {
    let mut buffer = *b"\x60\x86\x48\x01\x65\x03\x04\x01\x2A";
    let oid = AbsoluteOid::from_mut_bytes(&mut buffer).unwrap();
    let string = oid.to_string();
    assert_eq!(string, "2.16.840.1.101.3.4.1.42");
}

#[test]
fn relative_oid_from_mut_bytes() {
    let mut buffer = *b"\x86\x48\x01\x65\x03\x04\x01\x2A";
    let oid = RelativeOid::from_mut_bytes(&mut buffer).unwrap();
    let string = oid.to_string();
    assert_eq!(string, ".840.1.101.3.4.1.42");
}
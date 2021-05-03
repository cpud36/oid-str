use oid_str::RelativeOid;

#[test]
fn arc_boundary_at_start() {
    let oid = RelativeOid::from_bytes(b"\x01\x81\x00").unwrap();
    assert!(oid.is_arc_boundary(0));
}

#[test]
fn arc_boundary_after_the_end() {
    let oid = RelativeOid::from_bytes(b"\x81\x01\x81\x00").unwrap();
    assert!(oid.is_arc_boundary(4));
}

#[test]
fn arc_boundary_at_the_end() {
    let oid = RelativeOid::from_bytes(b"\x81\x01\x81\x00\x02").unwrap();
    assert!(oid.is_arc_boundary(4));
}

#[test]
fn arc_boundary_short_short() {
    let oid = RelativeOid::from_bytes(b"\x02\x01\x81\x00").unwrap();
    assert!(oid.is_arc_boundary(1));
}

#[test]
fn arc_boundary_short_long() {
    let oid = RelativeOid::from_bytes(b"\x02\x81\x01").unwrap();
    assert!(oid.is_arc_boundary(1));
}

#[test]
fn arc_boundary_long_long() {
    let oid = RelativeOid::from_bytes(b"\xaf\x02\x81\x01").unwrap();
    assert!(oid.is_arc_boundary(2));
}

#[test]
fn arc_boundary_not_at_last_byte() {
    let oid = RelativeOid::from_bytes(b"\xaf\x01\x02").unwrap();
    assert!(!oid.is_arc_boundary(1));
}

#[test]
fn arc_boundary_not_in_the_middle() {
    let oid = RelativeOid::from_bytes(b"\xaf\x88\x01\x02").unwrap();
    assert!(!oid.is_arc_boundary(1));
    assert!(!oid.is_arc_boundary(2));
}

#[test]
fn arc_boundary_not_after_the_end() {
    let oid = RelativeOid::from_bytes(b"\xaf\x88\x01\x02").unwrap();
    assert!(!oid.is_arc_boundary(5));
    assert!(!oid.is_arc_boundary(6));
    assert!(!oid.is_arc_boundary(usize::MAX));
}

#[test]
fn get_arc_at() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    assert_eq!(oid.get_arc_at(1), Some(0x4 * 0x80 + 0x2));
}

#[test]
fn get_arc_at_inside_boundary() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    assert_eq!(oid.get_arc_at(2), None);
}

#[test]
fn get_arc_at_the_end() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    assert_eq!(oid.get_arc_at(4), None);
}

#[test]
fn get_to_end() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    let part = &oid[1..];
    assert_eq!(part.as_bytes(), b"\x84\x02\x04");
    let part_get = oid.get(1..).unwrap();
    assert_eq!(part_get, part);
    let mut oid2 = oid.to_owned();
    let part_mut = &mut oid2[1..];
    assert_eq!(part_mut, part);
    let part_get_mut = &mut oid2[1..];
    assert_eq!(part_get_mut, part);
}

#[test]
fn get_up_to() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    let part = &oid[..3];
    assert_eq!(part.as_bytes(), b"\x01\x84\x02");
    let part_get = oid.get(..3).unwrap();
    assert_eq!(part_get, part);
    let mut oid2 = oid.to_owned();
    let part_mut = &mut oid2[..3];
    assert_eq!(part_mut, part);
    let part_get_mut = &mut oid2[..3];
    assert_eq!(part_get_mut, part);
}

#[test]
fn get_from_to() {
    let oid = RelativeOid::from_bytes(b"\x01\x84\x02\x04").unwrap();
    let part = &oid[1..3];
    assert_eq!(part.as_bytes(), b"\x84\x02");
    let part_get = oid.get(1..3).unwrap();
    assert_eq!(part_get, part);
    let mut oid2 = oid.to_owned();
    let part_mut = &mut oid2[1..3];
    assert_eq!(part_mut, part);
    let part_get_mut = &mut oid2[1..3];
    assert_eq!(part_get_mut, part);
}

#[test]
fn index_out_of_bounds_get() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x04").unwrap();
    assert!(oid.get(1..6).is_none());
}

#[test]
#[should_panic = "byte index 6 is out of bounds of `.1.49282.4`"]
fn index_out_of_bounds() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x04").unwrap();
    let _ = &oid[1..6];
}

#[test]
#[should_panic = "byte index 6 is out of bounds of `.1.49282.4`"]
fn index_start_out_of_bounds() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x04").unwrap();
    let _ = &oid[6..7];
}

#[test]
fn index_end_is_inside_arc_get() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x04").unwrap();
    assert!(oid.get(1..3).is_none());
}

#[test]
#[should_panic = "byte index 3 is not an arc boundary; it is inside of an arc 49282 (bytes 1..4) in `.1.49282.4`"]
fn index_end_is_inside_arc() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x04").unwrap();
    let _ = &oid[1..3];
}

#[test]
fn index_start_is_inside_arc_get() {
    let mut buffer = *b"\x01\x83\x81\x02\x04";
    let oid = RelativeOid::from_mut_bytes(&mut buffer).unwrap();
    assert!(oid.get(2..).is_none());
    assert!(oid.get_mut(2..).is_none());
}

#[test]
#[should_panic = "byte index 2 is not an arc boundary; it is inside of an arc 49282 (bytes 1..4) in `.1.49282.513.2`"]
fn index_start_is_inside_arc() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02\x84\x01\x02").unwrap();
    let _ = &oid[2..];
}

#[test]
fn index_start_gt_end_get() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02").unwrap();
    assert!(oid.get(4..1).is_none());
}

#[test]
#[should_panic = "start <= end (4 <= 1) when slicing `.1.49282`"]
fn index_start_gt_end() {
    let oid = RelativeOid::from_bytes(b"\x01\x83\x81\x02").unwrap();
    let _ = &oid[4..1];
}

#[test]
#[should_panic = ".128.128`[...]"]
fn index_long_oid_panic() {
    let mut buffer = Vec::default();
    while buffer.len() < 254 {
        buffer.extend_from_slice(b"\x81\x00");
    }
    assert_eq!(buffer.len(), 254);
    buffer.extend_from_slice(b"\x81\x80\x00");
    assert_eq!(buffer.len(), 257);
    let oid = RelativeOid::from_bytes(&buffer).unwrap();
    let _ = &oid[1..];
}

#[test]
#[should_panic = ".128.128`"]
fn index_long_oid_panic_exact() {
    let mut buffer = Vec::default();
    while buffer.len() < 254 {
        buffer.extend_from_slice(b"\x81\x00");
    }
    buffer.extend_from_slice(b"\x81\x00");
    assert_eq!(buffer.len(), 256);
    let oid = RelativeOid::from_bytes(&buffer).unwrap();
    let _ = &oid[1..];
}

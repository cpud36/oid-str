use oid_str::{AbsoluteOid, AbsoluteOidVec, Arc0, Arc1, RelativeOid, RootOid};

mod parse;
mod from_bytes;
mod encode;
mod iter;
mod display;
mod index;

#[test]
fn test_vec_from_root() {
    let root = RootOid::new(Arc0::Iso, Arc1::new(3).unwrap());
    let oid = AbsoluteOidVec::from_root(root);

    assert_eq!(oid.as_bytes(), b"\x2B");
}

#[test]
fn absolute_mut_tail() {
    let mut buffer1 = *b"\x60\x86\x48\x01\x65\x03\x04\x01\x2A";
    let mut buffer2 = buffer1;
    let absolute = AbsoluteOid::from_mut_bytes(&mut buffer1).unwrap();
    let relative = RelativeOid::from_mut_bytes(&mut buffer2[1..]).unwrap();
    assert_eq!(absolute.tail_mut(), relative);
}
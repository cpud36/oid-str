use crate::{Arc, RelativeOid, ARC_LEN};

pub fn write_b128(buf: &mut [u8; ARC_LEN], mut arc: Arc) -> &mut RelativeOid {
    let mut k = ARC_LEN - 1;
    buf[k] = (arc & 0x7f) as u8;
    arc >>= 7;
    while arc > 0 {
        k -= 1;
        buf[k] = 0x80 | (arc & 0x7f) as u8;
        arc >>= 7;
    }
    let bytes = &mut buf[k..];
    // SAFETY: bytes were just constructed, so it is a valid (1-segment) oid
    unsafe { RelativeOid::from_mut_bytes_unchecked(bytes) }
}

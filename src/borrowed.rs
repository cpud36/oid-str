#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{Arc, Arc0, Arc1, Position};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
/// Represents first two arcs of an OID.
///
/// # Invariants
/// First arc MUST be one of `0, 1, 2`(see [Arc0]),
/// second arc MUST be in range `0 ..= 39`(see [Arc1]).
/// Arcs are composed into a byte as `arc0 * 40 + arc1`,
/// Thus the stored byte must be within range `0..=119`(`0..=0x77`), see [MAX_ROOT_BYTE].
pub struct RootOid(u8);

pub const MAX_ROOT_BYTE: u8 = 40 * 2 + 39;

impl RootOid {
    /// # Safety
    /// `byte` must be within range `0..=119`(`0..=0x77`), see [RootOid]
    pub const unsafe fn from_u8_unchecked(byte: u8) -> RootOid {
        RootOid(byte)
    }

    pub const fn into_u8(self) -> u8 {
        self.0
    }

    const fn check(byte: u8) -> Result<(), RootError> {
        if byte > MAX_ROOT_BYTE {
            return Err(RootError(()));
        }
        Ok(())
    }

    pub const fn from_u8(byte: u8) -> Result<RootOid, RootError> {
        if let Err(e) = RootOid::check(byte) {
            return Err(e);
        }
        Ok(unsafe { RootOid::from_u8_unchecked(byte) })
    }

    pub const fn new(arc0: Arc0, arc1: Arc1) -> RootOid {
        let byte = arc0 as u8 * 40 + arc1.as_u8();
        // SAFETY: byte was just constructed,
        // `arc0` guarantees it is one of `0, 1, 2`,
        // `arc1` guarantees it is withing range `0..39`
        unsafe { RootOid::from_u8_unchecked(byte) }
    }

    pub fn as_absolute(&self) -> &AbsoluteOid {
        let bytes = core::slice::from_ref(&self.0);
        unsafe {
            // SAFETY: bytes are one element array
            // and the first byte invariants are exactly
            // the invariants of RootOid
            AbsoluteOid::from_bytes_unchecked(bytes)
        }
    }
}

#[derive(Debug, Clone)]
pub struct RootError(());

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
// FIXME: [CStr] is not marked with `#[repr(transparent)]` with a comment about being blocked on attribute privacy
//        Is it okay to mark it now?
// See: [CStr] impl in std for relative details
/// A slice (possibly empty) of [AbsoluteOid].
/// The slice must not contain the first 2 arcs.
///
/// Essentially, represents a sequence of non-negative numbers.
/// Each number in the list is refered to as a [Arc].
///
/// Each arc is encoded with base 128.
///
/// # Invariants
/// Last arc must be finished, that is, last byte has high bit **not** set.
///
/// Each arc must be within range of [Arc].
/// Leading zero bits must be discarded.
/// Byte `0x80` is disallowed
/// This makes it possible to represent any [Arc] value.
pub struct RelativeOid([u8]);

impl RelativeOid {
    /// Cast RelativeOid from bytes
    ///
    /// Mostly a workaround of lacking support of DSTs
    /// # Safety
    /// byte sequence must ensure invariants of RelativeOid are satisfied
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &RelativeOid {
        // SAFETY: Casting to RelativeOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `bytes`.
        &*(bytes as *const [u8] as *const RelativeOid)
    }

    /// Bytes of BER-encoded oid
    ///
    /// It is correct to append these bytes to both `RelativeOid` and `AbsoluteOid`
    ///
    /// `RelativeOid` can be reconstructed with `RelativeOid::from_mut_bytes_unchecked`
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Cast RelativeOid from bytes
    ///
    /// Mostly a workaround of lacking support of DSTs
    /// # Safety
    /// byte sequence must ensure invariants of RelativeOid are satisfied
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut RelativeOid {
        // Safe, because Oid is just a type Wrapper of [u8] and repr(transparent)
        // OidCast::<&[u8], &Self> { from: data }.to
        // unsafe { &*(data as *const [u8] as *const Oid) }

        // SAFETY: Casting to RelativeOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `bytes`.
        &mut *(bytes as *mut [u8] as *mut RelativeOid)
    }

    /// Bytes of BER-encoded oid
    ///
    /// It is correct to append these bytes to both `RelativeOid` and `AbsoluteOid`
    ///
    /// `RelativeOid` can be reconstructed with `RelativeOid::from_mut_bytes_unchecked`
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub(crate) fn check_bytes(bytes: &[u8]) -> Result<(), B128Error> {
        if let Some(error) = next_b128_error(bytes) {
            return Err(error);
        }
        Ok(())
    }

    // FIXME: add iterator over all errors (might be usefull for error correction, or reporting)

    pub fn from_bytes(bytes: &[u8]) -> Result<&RelativeOid, B128Error> {
        RelativeOid::check_bytes(bytes)?;
        // SAFETY: check above confirms that bytes are parsable into `Vec<Segment>`
        Ok(unsafe { RelativeOid::from_bytes_unchecked(bytes) })
    }

    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut RelativeOid, B128Error> {
        RelativeOid::check_bytes(bytes)?;
        // SAFETY: check above confirms that bytes are parsable into `Vec<Segment>`
        Ok(unsafe { RelativeOid::from_mut_bytes_unchecked(bytes) })
    }
}

impl RelativeOid {
    /// Creates a reference to empty oid
    ///
    /// Note: `RelativeOid` can be empty, contrary to [AbsoluteOid]
    pub fn empty() -> &'static RelativeOid {
        // SAFETY: relative oid can be empty
        unsafe { RelativeOid::from_bytes_unchecked(&[]) }
    }
}

#[cfg(feature = "alloc")]
impl From<&RelativeOid> for Box<RelativeOid> {
    fn from(s: &RelativeOid) -> Box<RelativeOid> {
        let boxed: Box<[u8]> = Box::from(s.as_bytes());
        // SAFETY: Casting to RelativeOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent)
        // [u8] comes from RelativeOid so it's invariants are satisfied
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut RelativeOid) }
    }
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
// DST implementation is the same as `RelativeOid`
/// Represents Oid, starting from the root.
///
/// Essentially, a sequence of non-negative numbers.
/// Each number is refered to as a [Arc].
///
/// # Invariants
/// Must contain at least 1 byte.
/// First byte value must satisfy [RootOid] invariants
///
/// The rest of the bytes(`&bytes[1..]`) must conform to [RelativeOid] invariants
pub struct AbsoluteOid([u8]);

impl AbsoluteOid {
    /// Cast `AbsoluteOid` from bytes
    ///
    /// Mostly a workaround of lacking support of DSTs
    /// # Safety
    /// byte sequence must ensure invariants of RelativeOid are satisfied
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &AbsoluteOid {
        // SAFETY: Casting to RelativeOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `bytes`.
        &*(bytes as *const [u8] as *const AbsoluteOid)
    }

    /// Bytes of BER-encoded oid
    ///
    /// `AbsoluteOid` can be reconstructed with `AbsoluteOid::from_bytes_unchecked`
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: Casting RelativeOid to [u8] is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `self`.
        unsafe { &*(self as *const AbsoluteOid as *const [u8]) }
    }

    /// Cast AbsoluteOid from bytes
    ///
    /// Mostly a workaround of lacking support of DSTs
    /// # Safety
    /// byte sequence must ensure invariants of AbsoluteOid are satisfied
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut Self {
        // Safe, because Oid is just a type Wrapper of [u8] and repr(transparent)
        // OidCast::<&[u8], &Self> { from: data }.to
        // unsafe { &*(data as *const [u8] as *const Oid) }

        // SAFETY: Casting to AbsoluteOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `bytes`.
        &mut *(bytes as *mut [u8] as *mut AbsoluteOid)
    }

    /// Bytes of BER-encoded oid
    ///
    /// `AbsoluteOid` can be reconstructed with `AbsoluteOid::from_mut_bytes_unchecked`
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        // SAFETY: Casting AbsoluteOid to [u8] is safe because its internal representation
        // is a [u8] too and it is repr(transparent).
        // Dereferencing the obtained pointer is safe because it comes from a
        // reference. Making a reference is then safe because its lifetime
        // is bound by the lifetime of the given `self`.
        unsafe { &mut *(self as *mut AbsoluteOid as *mut [u8]) }
    }

    pub(crate) fn check_bytes(bytes: &[u8]) -> Result<(), OidDecodingError> {
        if bytes.is_empty() {
            return Err(OidDecodingError::Empty);
        }
        let first_byte = bytes[0];
        RootOid::check(first_byte)?;
        RelativeOid::check_bytes(&bytes[1..])?;
        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<&AbsoluteOid, OidDecodingError> {
        AbsoluteOid::check_bytes(bytes)?;
        // SAFETY: check above confirms that bytes are parsable into `Vec<Arc>`
        Ok(unsafe { AbsoluteOid::from_bytes_unchecked(bytes) })
    }

    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut AbsoluteOid, OidDecodingError> {
        AbsoluteOid::check_bytes(bytes)?;
        // SAFETY: check above confirms that bytes are parsable into `Vec<Arc>`
        Ok(unsafe { AbsoluteOid::from_mut_bytes_unchecked(bytes) })
    }
}

#[cfg(feature = "alloc")]
impl From<&AbsoluteOid> for Box<AbsoluteOid> {
    fn from(s: &AbsoluteOid) -> Box<AbsoluteOid> {
        let boxed: Box<[u8]> = Box::from(s.as_bytes());
        // SAFETY: Casting to AbsoluteOid is safe because its internal representation
        // is a [u8] too and it is repr(transparent)
        // [u8] comes from AbsoluteOid so it's invariants are satisfied
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut AbsoluteOid) }
    }
}

impl AbsoluteOid {
    /// First two arcs
    pub fn root(&self) -> RootOid {
        if cfg!(debug_assertions) {
            RootOid::from_u8(self.as_bytes()[0]).unwrap();
        }
        // SAFETY: first byte is always present and contains a valid value for [RootOid]
        unsafe { RootOid::from_u8_unchecked(*self.as_bytes().get_unchecked(0)) }
    }

    /// All arcs except the first two(the [RootOid])
    pub fn tail(&self) -> &RelativeOid {
        if cfg!(debug_assertions) {
            RelativeOid::from_bytes(&self.as_bytes()[1..]).unwrap();
        }
        // SAFETY: `RootOid` always takes exactly 1 byte, so the rest is a valid `RelativeOid`
        unsafe { RelativeOid::from_bytes_unchecked(self.as_bytes().get_unchecked(1..)) }
    }

    /// All arcs except the first two(the [RootOid])
    pub fn tail_mut(&mut self) -> &mut RelativeOid {
        if cfg!(debug_assertions) {
            RelativeOid::from_mut_bytes(&mut self.as_mut_bytes()[1..]).unwrap();
        }
        // SAFETY: `RootOid` always takes exactly 1 byte, so the rest is a valid `RelativeOid`
        unsafe { RelativeOid::from_mut_bytes_unchecked(self.as_mut_bytes().get_unchecked_mut(1..)) }
    }
}

#[derive(Debug, Clone)]
pub enum OidDecodingError {
    Empty,
    Root(RootError),
    Base128(B128Error),
}

impl From<RootError> for OidDecodingError {
    fn from(error: RootError) -> Self {
        OidDecodingError::Root(error)
    }
}

impl From<B128Error> for OidDecodingError {
    fn from(error: B128Error) -> Self {
        OidDecodingError::Base128(error)
    }
}

#[derive(Debug, Clone)]
pub struct B128Error {
    pub kind: B128ErrorKind,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub enum B128ErrorKind {
    OutOfRange,
    ZeroByteWithCont,
    Unfinished,
}

fn next_b128_error(bytes: &[u8]) -> Option<B128Error> {
    const N_BYTES: u8 = core::mem::size_of::<Arc>() as u8;
    const FIRST_BYTE_N_BITS: u8 = (N_BYTES * 8) % 7;
    const FIRST_BYTE_MASK: u8 = 0x80 - (1 << FIRST_BYTE_N_BITS);

    // each byte stores 7 bits
    // whatever the `Arc` size is, it is not divisible by 7 (niether 8, 16, 32, 64, etc)
    // in order to be able to decode any Arc value,
    // we need to discard leading 0 bits in the first byte

    // FIXME: this might be a good candidate for simd optimisation
    let mut iter = bytes.iter().enumerate();
    while let Some((pos, &byte)) = iter.next() {
        let pos = pos as Position;

        match byte {
            0..=0x7f => continue,
            0x80 => {
                return Some(B128Error {
                    kind: B128ErrorKind::ZeroByteWithCont,
                    pos,
                })
            }
            _ => {}
        }

        // we slightly over/underestimate number of bits, but this is ok
        // if the number is valid, we underestimate the number of bits
        // if the number is invalid, we overestimate the number of bits
        //
        // suppose `Arc = u16`,
        // then mask = 0x80 - 0x04 = 0x7c = 0b0111_1100,
        // and there are the following posibilites:
        //      [7f      ] => first_byte = 1    n_bytes = 1    value = 0x007f
        //      [ff 7f   ] => first_byte = 1    n_bytes = 2    value = 0x3fff
        //      [ff ff 03] => first_byte = 0    n_bytes = 2    value = 0xffff
        //      [ff ff 04] => first_byte = 1    n_bytes = 3    invalid
        let first_byte = if byte & FIRST_BYTE_MASK == 0 { 0 } else { 1 };

        let mut n_bytes = first_byte;

        loop {
            match iter.next() {
                Some((_, &byte)) => {
                    n_bytes += 1;
                    if n_bytes > N_BYTES {
                        return Some(B128Error {
                            kind: B128ErrorKind::OutOfRange,
                            pos,
                        });
                    }
                    if byte < 0x80 {
                        break;
                    }
                }
                None => {
                    // We've exhausted `iter`, but haven't hit terminating byte,
                    // so we have an unfinished number
                    return Some(B128Error {
                        kind: B128ErrorKind::Unfinished,
                        pos,
                    });
                }
            }
        }
    }

    None
}

use core::ops::Deref;

use crate::{AbsoluteOid, RelativeOid, borrowed::OidDecodingError};



/// [AbsoluteOid] that can be constructed with `const fn`
/// 
/// # Invariants
/// stored bytes conform to [AbsoluteOid] invariants
pub struct StaticAbsoluteOid<'a> {
    bytes: &'a [u8],
}

impl<'a> Deref for StaticAbsoluteOid<'a> {
    type Target = AbsoluteOid;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.bytes` always satisfy invariants of [AbsoluteOid]
        //         this is ensured in [Self::from_bytes_unchecked]
        unsafe { AbsoluteOid::from_bytes_unchecked(self.bytes) }
    }
}

impl<'a> StaticAbsoluteOid<'a> {
    /// # Safety
    /// `bytes` must conform to [AbsoluteOid] invariants
    pub const unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> StaticAbsoluteOid<'a> {
        StaticAbsoluteOid { bytes }
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Result<StaticAbsoluteOid<'a>, OidDecodingError> {
        AbsoluteOid::check_bytes(bytes)?;
        // SAFETY: check above ensures invariants of AbsoluteOid are satisfied
        Ok(unsafe { StaticAbsoluteOid::from_bytes_unchecked(bytes) })
    }
}


/// [RelativeOid] that can be constructed with `const fn`
/// 
/// # Invariants
/// stored bytes conform to [AbsoluteOid] invariants
pub struct StaticRelativeOid<'a> {
    bytes: &'a [u8],
}

impl<'a> Deref for StaticRelativeOid<'a> {
    type Target = RelativeOid;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.bytes` always satisfy invariants of [RelativeOid]
        //         this is ensured in [Self::from_bytes_unchecked]
        unsafe { RelativeOid::from_bytes_unchecked(self.bytes) }
    }
}

impl<'a> StaticRelativeOid<'a> {
    /// # Safety
    /// `bytes` must conform to [RelativeOid] invariants
    pub const unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> StaticRelativeOid<'a> {
        StaticRelativeOid { bytes }
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Result<StaticRelativeOid<'a>, OidDecodingError> {
        RelativeOid::check_bytes(bytes)?;
        // SAFETY: check above ensures invariants of RelativeOid are satisfied
        Ok(unsafe { StaticRelativeOid::from_bytes_unchecked(bytes) })
    }
}



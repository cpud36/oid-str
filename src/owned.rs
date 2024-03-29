use alloc::vec::Vec;

use crate::{encode::write_b128, AbsoluteOid, Arc, RelativeOid, RootOid, ARC_LEN};

#[derive(Clone, PartialEq, Eq)]
pub struct RelativeOidVec {
    bytes: Vec<u8>,
}

impl Default for RelativeOidVec {
    fn default() -> Self {
        RelativeOidVec::from_oid(RelativeOid::empty())
    }
}

impl core::ops::Deref for RelativeOidVec {
    type Target = RelativeOid;

    fn deref(&self) -> &Self::Target {
        self.as_oid()
    }
}

impl core::ops::DerefMut for RelativeOidVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_oid()
    }
}

impl RelativeOidVec {
    pub fn from_oid(oid: &RelativeOid) -> RelativeOidVec {
        RelativeOidVec {
            bytes: oid.as_bytes().to_vec(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_oid(&self) -> &RelativeOid {
        // SAFETY: we only store valid RelativeOid bytes
        unsafe { RelativeOid::from_bytes_unchecked(self.bytes.as_slice()) }
    }

    pub fn as_mut_oid(&mut self) -> &mut RelativeOid {
        // SAFETY: we only store valid RelativeOid bytes
        unsafe { RelativeOid::from_mut_bytes_unchecked(self.bytes.as_mut_slice()) }
    }

    pub fn push(&mut self, arc: Arc) {
        let mut buffer = [0u8; ARC_LEN];
        let arc = write_b128(&mut buffer, arc);
        self.extend(arc);
    }

    pub fn extend(&mut self, oid: &RelativeOid) {
        self.bytes.extend(oid.as_bytes())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AbsoluteOidVec {
    bytes: Vec<u8>,
}

impl core::ops::Deref for AbsoluteOidVec {
    type Target = AbsoluteOid;

    fn deref(&self) -> &Self::Target {
        self.as_oid()
    }
}

impl core::ops::DerefMut for AbsoluteOidVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_oid()
    }
}

impl AbsoluteOidVec {
    pub fn from_root(root: RootOid) -> Self {
        AbsoluteOidVec::from_oid(root.as_absolute())
    }

    pub fn from_oid(oid: &AbsoluteOid) -> AbsoluteOidVec {
        AbsoluteOidVec {
            bytes: oid.as_bytes().to_vec(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_oid(&self) -> &AbsoluteOid {
        // SAFETY: we only store valid contents
        unsafe { AbsoluteOid::from_bytes_unchecked(self.bytes.as_slice()) }
    }

    pub fn as_mut_oid(&mut self) -> &mut AbsoluteOid {
        // SAFETY: we only store valid contents
        unsafe { AbsoluteOid::from_mut_bytes_unchecked(self.bytes.as_mut_slice()) }
    }

    pub fn push(&mut self, arc: Arc) {
        let mut buffer = [0u8; ARC_LEN];
        let arc = write_b128(&mut buffer, arc);
        self.extend(arc);
    }

    pub fn extend(&mut self, oid: &RelativeOid) {
        self.bytes.extend(oid.as_bytes())
    }
}

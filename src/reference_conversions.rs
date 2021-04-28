use core::borrow::{Borrow, BorrowMut};

use crate::{AbsoluteOid, AbsoluteOidVec, RelativeOid, RelativeOidVec};

impl AsRef<AbsoluteOid> for AbsoluteOidVec {
    fn as_ref(&self) -> &AbsoluteOid {
        self.as_oid()
    }
}

impl Borrow<AbsoluteOid> for AbsoluteOidVec {
    fn borrow(&self) -> &AbsoluteOid {
        self.as_oid()
    }
}

impl BorrowMut<AbsoluteOid> for AbsoluteOidVec {
    fn borrow_mut(&mut self) -> &mut AbsoluteOid {
        self.as_mut_oid()
    }
}

impl AsRef<RelativeOid> for RelativeOidVec {
    fn as_ref(&self) -> &RelativeOid {
        self.as_oid()
    }
}

impl Borrow<RelativeOid> for RelativeOidVec {
    fn borrow(&self) -> &RelativeOid {
        self.as_oid()
    }
}

impl BorrowMut<RelativeOid> for RelativeOidVec {
    fn borrow_mut(&mut self) -> &mut RelativeOid {
        self.as_mut_oid()
    }
}

#[cfg(feature = "alloc")]
mod with_alloc {
    use super::*;
    use alloc::borrow::ToOwned;

    impl ToOwned for AbsoluteOid {
        type Owned = AbsoluteOidVec;

        fn to_owned(&self) -> Self::Owned {
            AbsoluteOidVec::from_oid(self)
        }
    }

    impl ToOwned for RelativeOid {
        type Owned = RelativeOidVec;

        fn to_owned(&self) -> Self::Owned {
            RelativeOidVec::from_oid(self)
        }
    }
}

impl AsRef<[u8]> for AbsoluteOid {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for RelativeOid {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod root;
mod borrowed;
mod iter;
mod encode;
#[cfg(feature = "alloc")]
mod owned;
mod str;

pub use self::str::{parse_absolute, parse_relative, OidParsingError};
pub use borrowed::{
    AbsoluteOid, B128Error, B128ErrorKind, OidDecodingError, RelativeOid, RootError, RootOid,
    MAX_ROOT_BYTE,
};
pub use encode::write_b128;
pub use iter::{AbsoluteArcs, RelativeArcs, RootArcs};
#[cfg(feature = "alloc")]
pub use owned::{AbsoluteOidVec, RelativeOidVec};
pub use root::{Arc0, Arc1};

pub type Arc = u32;
pub type Position = u16;
pub const ARC_LEN: usize = (core::mem::size_of::<Arc>() * 8 + 6) / 7;

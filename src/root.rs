use core::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arc0 {
    ItuT = 0,
    Iso = 1,
    JointIsoItuT = 2,
}

impl Arc0 {
    pub const fn new(value: u8) -> Result<Arc0, IllegalRootNodeError> {
        match value {
            0 => Ok(Arc0::ItuT),
            1 => Ok(Arc0::Iso),
            2 => Ok(Arc0::JointIsoItuT),
            _ => Err(IllegalRootNodeError(())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IllegalRootNodeError(());

impl TryFrom<u8> for Arc0 {
    type Error = IllegalRootNodeError;
    fn try_from(value: u8) -> Result<Arc0, Self::Error> {
        Arc0::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arc1(u8);

impl Arc1 {
    /// # Safety
    /// value must be < 40
    pub const unsafe fn new_unchecked(value: u8) -> Arc1 {
        Arc1(value)
    }

    pub const fn new(value: u8) -> Result<Arc1, IllegalArc1Error> {
        match value {
            0..=39 => unsafe {
                Ok(Arc1::new_unchecked(value))
            },
            _ => Err(IllegalArc1Error(())),
        }
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct IllegalArc1Error(());

impl TryFrom<u8> for Arc1 {
    type Error = IllegalArc1Error;
    fn try_from(value: u8) -> Result<Arc1, Self::Error> {
        Arc1::new(value)
    }
}

impl From<Arc1> for u8 {
    fn from(arc: Arc1) -> Self {
        arc.as_u8()
    }
}

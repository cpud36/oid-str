use core::convert::TryFrom;

use crate::{borrowed::MAX_ROOT_BYTE, AbsoluteOid, Arc, Arc0, Arc1, RelativeOid, RootOid};

impl AbsoluteOid {
    pub fn arcs(&self) -> AbsoluteArcs {
        AbsoluteArcs {
            root: self.root().arcs(),
            tail: self.tail().arcs(),
        }
    }
}

impl RootOid {
    pub const fn arcs(&self) -> RootArcs {
        RootArcs {
            byte: self.into_u8(),
        }
    }

    pub fn into_arcs(self) -> (Arc0, Arc1) {
        let byte = self.into_u8();
        let arc0 = Arc0::try_from(byte / 40).unwrap();
        let arc1 = Arc1::try_from(byte % 40).unwrap();
        (arc0, arc1)
    }
}

impl RelativeOid {
    pub fn arcs(&self) -> RelativeArcs {
        RelativeArcs {
            bytes: self.as_bytes().iter(),
        }
    }
}

pub struct RootArcs {
    /// Value higher than `119(0x77)` means `segment0` has been consumed,
    /// Value `0xff` means all bits has been consumed
    byte: u8,
}

pub struct AbsoluteArcs<'a> {
    root: RootArcs,
    tail: RelativeArcs<'a>,
}

pub struct RelativeArcs<'a> {
    bytes: core::slice::Iter<'a, u8>,
}

impl core::iter::Iterator for RootArcs {
    type Item = Arc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte == 0xff {
            return None;
        }
        let arc = if self.byte > MAX_ROOT_BYTE {
            let byte = core::mem::replace(&mut self.byte, 0xff);
            byte % 40
        } else {
            let arc = self.byte / 40;
            let old_byte = self.byte;
            // we need to preserve `segment1 = byte % 40`, and we do not want to overflow
            self.byte += (4 - arc) * 40;

            debug_assert_eq!(
                old_byte % 40,
                self.byte % 40,
                "segment1 was changed when consuming segment0"
            );
            debug_assert!(self.byte > MAX_ROOT_BYTE, "failed to consume segment0");

            arc
        };
        Some(arc as Arc)
    }
}

impl core::iter::Iterator for RelativeArcs<'_> {
    type Item = Arc;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accumulator = 0;
        while let Some(&byte) = self.bytes.next() {
            if !b128_eat_byte(&mut accumulator, byte) {
                return Some(accumulator);
            }
        }
        debug_assert_eq!(accumulator, 0, "cannot have unfinished segment");
        None
    }
}

impl core::iter::Iterator for AbsoluteArcs<'_> {
    type Item = Arc;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.root.next() {
            return Some(s);
        }
        self.tail.next()
    }
}

fn b128_eat_byte(accumulator: &mut Arc, byte: u8) -> bool {
    // We won't overflow because we guarantee, we feed only valid bytes to absolute oid
    *accumulator *= 1 << 7;
    *accumulator += (byte & !0x80) as Arc;

    (byte & 0x80) != 0
}

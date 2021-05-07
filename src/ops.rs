use core::ops::RangeBounds;

use crate::{index::slice_error_fail, AbsoluteOid, RelativeOid};

impl RelativeOid {
    pub fn split_at(&self, mid: usize) -> (&RelativeOid, &RelativeOid) {
        if self.is_arc_boundary(mid) {
            let (first, last) = self.as_bytes().split_at(mid);

            unsafe {
                // SAFETY: just checked mid is on the char boundary,
                //         `first` does not include mid, and `last` starts from mid
                let first = RelativeOid::from_bytes_unchecked(first);
                let last = RelativeOid::from_bytes_unchecked(last);

                (first, last)
            }
        } else {
            slice_error_fail(self, 0, mid)
        }
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut RelativeOid, &mut RelativeOid) {
        if self.is_arc_boundary(mid) {
            unsafe {
                // SAFETY: `first` and `last` are immediatly wrapped again as RelativeOid and do not escape current function
                let (first, last) = self.as_mut_bytes().split_at_mut(mid);

                // SAFETY: just checked mid is on the char boundary,
                //         `first` does not include mid, and `last` starts from mid
                let first = RelativeOid::from_mut_bytes_unchecked(first);
                let last = RelativeOid::from_mut_bytes_unchecked(last);

                (first, last)
            }
        } else {
            slice_error_fail(self, 0, mid)
        }
    }

    pub fn starts_with(&self, prefix: &RelativeOid) -> bool {
        // if we get a hit, we can be sure prefix end is on the arc boudnary
        self.as_bytes().starts_with(prefix.as_bytes())
    }

    pub fn ends_with(&self, suffix: &RelativeOid) -> bool {
        // if ends_with returns true, suffix.len <= self.len
        // the second check is required to catch the following "81 81 01".ends_with("81 01")
        self.as_bytes().ends_with(suffix.as_bytes())
            && self.is_arc_boundary(self.len() - suffix.len())
    }

    pub fn strip_prefix(&self, prefix: &RelativeOid) -> Option<&RelativeOid> {
        if self.starts_with(prefix) {
            // SAFETY: self contains at least prefix bytes
            //         AND there is an arc boundary at prefix end
            debug_assert!(self.is_arc_boundary(prefix.len()));
            let rest = unsafe { self.get_unchecked(prefix.len()..) };
            Some(rest)
        } else {
            None
        }
    }

    pub fn strip_suffix(&self, suffix: &RelativeOid) -> Option<&RelativeOid> {
        if self.ends_with(suffix) {
            // SAFETY: self contains at least suffix bytes
            //         AND there is an arc boundary at suffix bytes from the end
            debug_assert!(self.is_arc_boundary(self.len() - suffix.len()));
            let head = unsafe { self.get_unchecked(..self.len() - suffix.len()) };
            Some(head)
        } else {
            None
        }
    }

    pub fn copy_from(&mut self, src: &RelativeOid) {
        unsafe {
            let dst = self.as_mut_bytes();
            let src = src.as_bytes();

            dst.copy_from_slice(src);
        }
    }

    pub(crate) unsafe fn copy_within<R: RangeBounds<usize>>(&mut self, src: R, dest: usize) {
        let dst = self.as_mut_bytes();

        dst.copy_within(src, dest);
    }
}

impl AbsoluteOid {
    pub fn starts_with(&self, prefix: &AbsoluteOid) -> bool {
        self.root() == prefix.root() && self.tail().starts_with(prefix.tail())
    }

    pub fn ends_with(&self, suffix: &RelativeOid) -> bool {
        self.tail().ends_with(suffix)
    }

    pub fn strip_prefix(&self, prefix: &AbsoluteOid) -> Option<&RelativeOid> {
        if self.root() == prefix.root() {
            self.tail().strip_prefix(prefix.tail())
        } else {
            None
        }
    }

    pub fn strip_suffix(&self, suffix: &RelativeOid) -> Option<&AbsoluteOid> {
        unsafe {
            let tail = self.tail().strip_suffix(suffix)?;
            // SAFETY: tail is a sublice of self.tail(), and therefore a subslice of self
            //         strip_suffix does not alter start ptr
            Some(self.with_tail_unchecked(tail))
        }
    }
}

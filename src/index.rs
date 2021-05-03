use core::ops::{Index, IndexMut, RangeBounds};

use crate::{AbsoluteOid, Arc, RelativeOid};

/// [`AbsoluteOid`] is inconvenient for slicing.
/// First byte of [`AbsoluteOid`] has different meaning from the rest.
/// This makes it hard to return references.
/// 
/// For most use cases it is sufficient to get [`AbsoluteOid::tail`]
/// and use indexing functions for [`RelativeOid`], e.g. [`RelativeOid::get`]
/// 
impl AbsoluteOid {
    /// Length of the oid in bytes
    /// 
    /// Note, [`AbsoluteOid`] is **never empty**, thus `self.len() >= 1`
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }
}

impl RelativeOid {
    /// Length of the oid in bytes
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if there is there is a boundary between two arcs at `index`
    /// 
    /// It returns true if index points to the first byte of an arc.
    /// It is assumed, there is an arc after the end
    /// 
    /// # Examples
    /// ```
    /// # use oid_str::RelativeOid;
    /// 
    /// let oid = RelativeOid::from_bytes(b"\x81\x01\x02").unwrap();
    /// // First arc starts at oid[0]    81 01 02
    /// //                              |^^   | <- arc segment
    /// assert!(oid.is_arc_boundary(0));
    /// // Second arc starts at oid[1]   81 01 02
    /// //                                    |^^| <- arc segment
    /// assert!(oid.is_arc_boundary(2));
    /// // Arc end is not counted as arc boundary   81 01 02
    /// //                                         |   ^^| <- not first byte
    /// assert!(!oid.is_arc_boundary(1));
    /// // But it is assumed, there is an arc after the end     81 01 02 __
    /// //                                                     |     |  |^^ <- counted as an arc
    /// assert!(oid.is_arc_boundary(3));
    /// // It is safe to call with index out of bounds          81 01 02 __ __
    /// //                                                                  ^^ <- no arc here
    /// assert!(!oid.is_arc_boundary(4));
    /// ```
    pub fn is_arc_boundary(&self, index: usize) -> bool {
        if index == 0 || index == self.len() {
            return true;
        }
        match self.as_bytes().get(index - 1) {
            None => false,
            // previous byte is not a continuation byte
            Some(b) => b & 0x80 == 0,
        }
    }

    /// Get arc value at given offset(in bytes)
    /// 
    /// ```
    /// # use oid_str::RelativeOid;
    /// 
    /// let oid = RelativeOid::from_bytes(b"\x81\x01\x02\x82\x00").unwrap();
    /// assert_eq!(oid.get_arc_at(3), Some(2 * 1<<7));
    /// 
    /// assert_eq!(oid.get_arc_at(1), None);
    /// assert_eq!(oid.get_arc_at(5), None);
    /// ```
    pub fn get_arc_at(&self, i: usize) -> Option<Arc> {
        self.get(i..).and_then(|r| r.arcs().next())
    }

    /// Get slice at given range(in bytes)
    /// 
    /// Equivalent to slicing underlying byte slice and wrapping back into RelativeOid,
    /// but faster.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use oid_str::RelativeOid;
    /// 
    /// let oid = RelativeOid::from_bytes(b"\x81\x01\x02\x82\x00").unwrap();
    /// assert_eq!(oid.get(2..).unwrap().as_bytes(), b"\x02\x82\x00");
    /// // Arc boundary
    /// assert_eq!(oid.get(1..), None);
    /// ```
    pub fn get<I>(&self, range: I) -> Option<&RelativeOid>
    where
        I: RangeBounds<usize>,
    {
        let (start, end) = index_bounds(self, &range);
        if start <= end
            && self.is_arc_boundary(start)
            && self.is_arc_boundary(end) {
            Some(unsafe { self.get_unchecked(range) })
        } else {
            None
        }
    }
    
    /// Get mutable slice at given range(in bytes)
    /// 
    /// Same as [RelativeOid::get], but returns `&mut` reference
    pub fn get_mut<I>(&mut self, range: I) -> Option<&mut RelativeOid>
    where
        I: RangeBounds<usize>,
    {
        let (start, end) = index_bounds(self, &range);
        if start <= end
            && self.is_arc_boundary(start)
            && self.is_arc_boundary(end) {
            Some(unsafe { self.get_unchecked_mut(range) })
        } else {
            None
        }
    }

    /// Get slice at given range(in bytes) without doing bounds checks and arc boundary checks
    /// 
    /// For a safe alternative see [`get`]
    /// 
    /// # Safety
    /// 
    /// Callers of this method must ensure the following preconditions to be hold:
    /// 
    /// * range is withing the bounds of underlying slice: `0 <= range.start <= range.end <= self.len()`
    /// * both range ends lie on the arc boudnary(see [`is_arc_boundary`])
    /// 
    /// Failing these might produce references to invalid memory or
    /// might violate invariants of RelativeOid
    /// 
    /// [`get`]: RelativeOid::get
    /// [`is_arc_boundary`]: RelativeOid::is_arc_boundary
    pub unsafe fn get_unchecked<R>(&self, range: R) -> &RelativeOid
    where R: RangeBounds<usize> {
        let (start, end) = index_bounds(self, &range);
        let bytes = self.as_bytes().get_unchecked(start..end);
        RelativeOid::from_bytes_unchecked(bytes)
    }

    /// Get slice at given range(in bytes) without doing bounds checks and arc boundary checks
    /// 
    /// For a safe alternative see [`get`]
    /// 
    /// # Safety
    /// 
    /// The same preconditions apply as in [`get_unchecked`]
    /// 
    /// [`get`]: RelativeOid::get
    /// [`get_unchecked`]: RelativeOid::get_unchecked
    pub unsafe fn get_unchecked_mut<R>(&mut self, range: R) -> &mut RelativeOid
    where R: RangeBounds<usize> {
        let (start, end) = index_bounds(self, &range);
        let bytes = self.as_mut_bytes().get_unchecked_mut(start..end);
        RelativeOid::from_mut_bytes_unchecked(bytes)
    }
}

fn index_bounds<R>(oid: &RelativeOid, range: &R) -> (usize, usize)
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        core::ops::Bound::Included(&s) => s,
        core::ops::Bound::Excluded(&s) => s + 1,
        core::ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        core::ops::Bound::Included(&s) => s + 1,
        core::ops::Bound::Excluded(&s) => s,
        core::ops::Bound::Unbounded => oid.len(),
    };
    (start, end)
}

/// Panicing indexing operations for RelativeOid
/// 
/// Similar to indexing byte slices with ranges,
/// but panics if range ends do not lie on an arc boundary.
impl<R: RangeBounds<usize>> Index<R> for RelativeOid {
    type Output = RelativeOid;

    fn index(&self, index: R) -> &Self::Output {
        let (start, end) = index_bounds(self, &index);
        match self.get(index) {
            Some(oid) => oid,
            None => slice_error_fail(self, start, end),
        }
    }
}

impl<R: RangeBounds<usize>> IndexMut<R> for RelativeOid {
    fn index_mut(&mut self, index: R) -> &mut Self::Output {
        let (start, end) = index_bounds(self, &index);
        if start <= end
            && self.is_arc_boundary(start)
            && self.is_arc_boundary(end) {
            unsafe { self.get_unchecked_mut(index) }
        } else {
            slice_error_fail(self, start, end)
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn slice_error_fail(oid: &RelativeOid, start: usize, end: usize) -> ! {
    const MAX_DISPLAY_LENGTH: usize = 256;
    let (truncated, oid_trunc) = truncate_to_arc_boundary(oid, MAX_DISPLAY_LENGTH);
    let ellipsis = if truncated { "[...]" } else { "" };

    // 1. out of bounds
    if start > oid.len() || end > oid.len() {
        let oob_index = if start > oid.len() { start } else { end };
        panic!(
            "byte index {} is out of bounds of `{}`{}",
            oob_index, oid_trunc, ellipsis
        );
    }

    // 2. begin <= end
    assert!(
        start <= end,
        "start <= end ({} <= {}) when slicing `{}`{}",
        start,
        end,
        oid_trunc,
        ellipsis
    );

    // 3. arc boundary
    let index = if !oid.is_arc_boundary(start) {
        start
    } else {
        end
    };
    // find the arc
    let mut arc_start = index;
    while !oid.is_arc_boundary(arc_start) {
        arc_start -= 1;
    }
    // `arc_start` must be less than len and a char boundary
    let arc = oid[arc_start..].arcs().next().unwrap();
    let arc_range = arc_start..arc_start + b128_len(arc);
    panic!(
        "byte index {} is not an arc boundary; it is inside of an arc {:?} (bytes {:?}) in `{}`{}",
        index, arc, arc_range, oid_trunc, ellipsis
    );
}

fn truncate_to_arc_boundary(oid: &RelativeOid, mut max: usize) -> (bool, &RelativeOid) {
    if oid.len() <= max {
        (false, oid)
    } else {
        while !oid.is_arc_boundary(max) {
            max -= 1;
        }
        (true, &oid[..max])
    }
}

fn b128_len(arc: Arc) -> usize {
    match arc {
        0..=0x7f => 1,
        0x80..=0x3fff => 2,
        0x4000..=0x1f_ffff => 3,
        0x20_0000..=0xfff_ffff => 4,
        0x1000_0000..=0xffff_ffff => 5,
    }
}

//! Generic gobbler, like a more sophisticated peekable iterator.

/// Type for getting windows over a slice and sensibly gobbling them.
///
/// It's like a cursor type, but optimized around slicing buffers.
pub(crate) struct Gobbler<'b, T> {
    buf: &'b [T],
    at: usize,
}

impl<'b, T> Gobbler<'b, T> {
    pub(crate) fn new(buf: &'b [T]) -> Self {
        Self { buf, at: 0 }
    }

    pub(crate) fn len(&self) -> usize {
        self.buf.len()
    }

    pub(crate) fn at(&self) -> usize {
        self.at
    }

    pub(crate) fn remaining(&self) -> usize {
        self.buf.len() - self.at
    }

    /// Gets if there's an entry to be processed.
    pub(crate) fn has_entry(&self) -> bool {
        self.remaining() > 0
    }

    pub(crate) fn view(&self) -> &[T] {
        &self.buf[self.at..]
    }

    /// Consumes one entry.
    ///
    /// # Panics
    ///
    /// If `.has_entry()` return false.
    pub(crate) fn gobble_one(&mut self) {
        self.gobble_exact(1);
    }

    /// Gobbles an exact number of entries.
    ///
    /// # Panics
    ///
    /// If we run over the end of the input.
    pub(crate) fn gobble_exact(&mut self, n: usize) {
        let new_at = self.at() + n;
        if new_at > self.len() {
            panic!(
                "gobbler: overflow buffer (gobble {n}, new {new_at}, max {})",
                self.len()
            );
        }
        self.at = new_at;
    }

    /// Tries to gobble an entry satisfying a specific predicate.
    ///
    /// Returns if we gobbled the thing.
    pub(crate) fn _try_gobble(&mut self, cond: impl Fn(&T) -> bool) -> bool {
        if self.remaining() > 0 {
            let cur = self.get_expect();
            let ok = cond(cur);
            if ok {
                self.gobble_one();
            }
            ok
        } else {
            false
        }
    }

    /// Gobbles entries until the condition returns true or we run out of entries.
    ///
    /// Returns if we met the condition (true), or if we ran out of items (false).
    pub(crate) fn gobble_until(&mut self, cond: impl Fn(&T) -> bool) -> bool {
        while self.has_entry() {
            let entry = self.get_expect();
            if cond(entry) {
                return true;
            }

            self.gobble_one();
        }

        false
    }

    /// Scans forwards until finding an entry satisfying a condition.  Gobbles
    /// those entries and returns a slice over those entries.  Returns `None` if
    /// we never find a satisfing entry before reaching the end of the entries,
    /// resetting the current gobble index.
    ///
    /// The returned slice does not contain the entry that satisfied the
    /// condition.  The `.get()` entry will be the entry that satisfied the
    /// condition.
    #[deprecated(note = "use gobble_slice_up_to_or_end instead")]
    #[expect(dead_code, reason = "deprecated")]
    pub(crate) fn gobble_slice_up_to(&mut self, cond: impl Fn(&T) -> bool) -> Option<&[T]> {
        let start = self.at;

        while self.has_entry() {
            let e = self.get_expect();

            if cond(e) {
                return Some(&self.buf[start..self.at]);
            }

            self.gobble_one();
        }

        self.at = start;
        None
    }

    /// Scans forwards until finding an entry satisfying a condition. Gobbles
    /// those entries and returns a slice over those entries. If we reach the
    /// end of the entries without finding a satisfying entry, returns all
    /// remaining entries from the start position to the end.
    ///
    /// The returned slice does not contain the entry that satisfied the
    /// condition (if found).  The `.get()` entry will be the entry that satisfied
    /// the condition, or `None` if we reached the end.
    pub(crate) fn gobble_slice_up_to_or_end(&mut self, cond: impl Fn(&T) -> bool) -> &[T] {
        let start = self.at;

        while self.has_entry() {
            let e = self.get_expect();

            if cond(e) {
                return &self.buf[start..self.at];
            }

            self.gobble_one();
        }

        // Reached the end without finding the condition - return all remaining
        &self.buf[start..self.at]
    }

    /// Gets the item we're currently at.
    ///
    /// # Panics
    ///
    /// If `.has_entry()` return false.
    pub(crate) fn get_expect(&self) -> &T {
        &self.buf[self.at]
    }

    /// Gets the item we're currently at, if there is one.
    pub(crate) fn get(&self) -> Option<&T> {
        self.buf.get(self.at)
    }
}

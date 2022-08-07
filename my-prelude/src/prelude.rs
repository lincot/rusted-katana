pub trait PushUnchecked<T> {
    /// # Safety
    ///
    /// `length` needs to be equal to `capacity`
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        if self.len() == self.capacity() {
            core::hint::unreachable_unchecked();
        }
        self.push(value);
    }
}

impl PushUnchecked<char> for String {
    #[inline]
    unsafe fn push_unchecked(&mut self, ch: char) {
        match ch.len_utf8() {
            1 => self.as_mut_vec().push_unchecked(ch as u8),
            _ => {
                self.as_mut_vec()
                    .extend_from_slice_unchecked(ch.encode_utf8(&mut [0; 4]).as_bytes());
            }
        }
    }
}

pub trait ExtendUnchecked<T, I> {
    /// # Safety
    ///
    /// `length + iter.length` needs to be less than or equal to `capacity`
    unsafe fn extend_unchecked(&mut self, iter: I);
}

impl<I: IntoIterator<Item = char>> ExtendUnchecked<char, I> for String {
    #[inline]
    unsafe fn extend_unchecked(&mut self, iter: I) {
        for value in iter {
            self.push_unchecked(value);
        }
    }
}

pub trait ExtendFromSliceUnchecked<T> {
    /// # Safety
    ///
    /// `length + other.length` needs to be less than or equal to `capacity`
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]);
}

impl<T: Copy> ExtendFromSliceUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let count = other.len();
        let len = self.len();
        std::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
        self.set_len(len + count);
    }
}

pub trait PushStrUnchecked {
    /// # Safety
    ///
    /// `length + string.length` needs to be less than or equal to `capacity`
    unsafe fn push_str_unchecked(&mut self, string: &str);
}

impl PushStrUnchecked for String {
    #[inline]
    unsafe fn push_str_unchecked(&mut self, string: &str) {
        self.as_mut_vec()
            .extend_from_slice_unchecked(string.as_bytes());
    }
}

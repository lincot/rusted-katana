use core::mem::MaybeUninit;

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
                self.as_mut_vec().extend_from_slice_unchecked(
                    ch.encode_utf8(&mut MaybeUninit::<[u8; 4]>::uninit().assume_init())
                        .as_bytes(),
                );
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
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
        self.set_len(len + count);
    }
}

#[cfg(feature = "heapless")]
impl<T: Copy, const N: usize> ExtendFromSliceUnchecked<T> for heapless::Vec<T, N> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let count = other.len();
        let len = self.len();
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
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

pub trait WriteNumUnchecked {
    /// # Safety
    ///
    /// `length + n.to_string().len()` needs to be less than or equal to `capacity`
    #[cfg(feature = "lexical-core")]
    unsafe fn write_num_unchecked(&mut self, n: impl lexical_core::ToLexical);

    /// # Safety
    ///
    /// `length + n.to_string().len()` needs to be less than or equal to `capacity`
    #[cfg(not(feature = "lexical-core"))]
    unsafe fn write_num_unchecked(&mut self, n: impl ToString);
}

impl WriteNumUnchecked for Vec<u8> {
    #[cfg(feature = "lexical-core")]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl lexical_core::ToLexical) {
        let len = self.len();
        let written_len = lexical_core::write_unchecked(
            n,
            core::slice::from_raw_parts_mut(self.as_mut_ptr().add(len), self.capacity() - len),
        )
        .len();
        self.set_len(len + written_len);
    }

    #[cfg(not(feature = "lexical-core"))]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl ToString) {
        self.extend_from_slice_unchecked(n.to_string().as_bytes());
    }
}

#[cfg(feature = "heapless")]
impl<const N: usize> WriteNumUnchecked for heapless::Vec<u8, N> {
    #[cfg(feature = "lexical-core")]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl lexical_core::ToLexical) {
        let len = self.len();
        let written_len = lexical_core::write_unchecked(
            n,
            core::slice::from_raw_parts_mut(self.as_mut_ptr().add(len), self.capacity() - len),
        )
        .len();
        self.set_len(len + written_len);
    }

    #[cfg(not(feature = "lexical-core"))]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl ToString) {
        self.extend_from_slice_unchecked(n.to_string().as_bytes());
    }
}

impl WriteNumUnchecked for String {
    #[cfg(feature = "lexical-core")]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl lexical_core::ToLexical) {
        self.as_mut_vec().write_num_unchecked(n);
    }

    #[cfg(not(feature = "lexical-core"))]
    #[inline]
    unsafe fn write_num_unchecked(&mut self, n: impl ToString) {
        self.push_str_unchecked(&n.to_string());
    }
}

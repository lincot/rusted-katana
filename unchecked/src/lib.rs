#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub trait PushUnchecked<T> {
    /// # Safety
    ///
    /// `self.len()` must be `< self.capacity()`
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.len() < self.capacity());
        if self.len() >= self.capacity() {
            core::hint::unreachable_unchecked();
        }
        self.push(value);
    }
}

impl PushUnchecked<char> for String {
    /// # Safety
    ///
    /// `self.len() + ch.len_utf8()` must be `<= self.capacity()`
    #[inline]
    unsafe fn push_unchecked(&mut self, ch: char) {
        let len = self.len();
        let ptr = self.as_mut_vec().as_mut_ptr().add(len);
        let count = ch.len_utf8();
        debug_assert!(len + count <= self.capacity());
        match count {
            1 => {
                core::ptr::write(ptr, ch as u8);
            }
            2 => {
                core::ptr::write(ptr, (ch as u32 >> 6 & 0x1F) as u8 | 0b1100_0000);
                core::ptr::write(ptr.add(1), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            3 => {
                core::ptr::write(ptr, (ch as u32 >> 12 & 0x0F) as u8 | 0b1110_0000);
                core::ptr::write(ptr.add(1), (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(2), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            4 => {
                core::ptr::write(ptr, (ch as u32 >> 18 & 0x07) as u8 | 0b1111_0000);
                core::ptr::write(ptr.add(1), (ch as u32 >> 12 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(2), (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000);
                core::ptr::write(ptr.add(3), (ch as u32 & 0x3F) as u8 | 0b1000_0000);
            }
            _ => core::hint::unreachable_unchecked(),
        }
        self.as_mut_vec().set_len(len + count);
    }
}

pub trait ExtendUnchecked<T, I> {
    /// # Safety
    ///
    /// `self.len() + iter.count()` must be `<= self.capacity()`
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
    /// `other.len()` must be `<= self.capacity() - self.len()`
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSliceUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let len = self.len();
        let count = other.len();
        debug_assert!(count <= self.capacity() - len);
        if count > self.capacity() - len {
            core::hint::unreachable_unchecked();
        }
        self.extend_from_slice(other);
    }
}

#[cfg(feature = "heapless")]
impl<T: Copy, const N: usize> ExtendFromSliceUnchecked<T> for heapless::Vec<T, N> {
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let len = self.len();
        let count = other.len();
        debug_assert!(count <= self.capacity() - len);
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
        self.set_len(len + count);
    }
}

pub trait ExtendFromWithinUnchecked {
    /// # Safety
    ///
    /// - `src` needs to be valid index
    /// - `self.capacity() - self.len()` must be `>= src.len()`
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>);
}

impl<T: Copy> ExtendFromWithinUnchecked for Vec<T> {
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>) {
        let count = src.len();
        debug_assert!(src.start <= src.end || src.end <= self.len());
        debug_assert!(self.capacity() - self.len() >= count);
        let source = self.get_unchecked(src);
        core::ptr::copy_nonoverlapping(source.as_ptr(), self.as_mut_ptr().add(self.len()), count);
        self.set_len(self.len() + count);
    }
}

pub trait PushStrUnchecked {
    /// # Safety
    ///
    /// `self.len() + string.len()` must be `<= self.capacity()`
    unsafe fn push_str_unchecked(&mut self, string: &str);
}

impl PushStrUnchecked for String {
    #[inline]
    unsafe fn push_str_unchecked(&mut self, string: &str) {
        self.as_mut_vec()
            .extend_from_slice_unchecked(string.as_bytes());
    }
}

#![no_std]

pub trait VqSort: Sized {
    fn sort(data: &mut [Self]);
    fn sort_descending(data: &mut [Self]);
}

macro_rules! vqsort_impl {
    ($($t:ty)*) => ($(
        paste::paste! {
            extern "C" {
                fn [<vqsort_ $t>](data: *mut $t, len: usize);
                fn [<vqsort_ $t _descending>](data: *mut $t, len: usize);
            }

            impl VqSort for $t {
                #[inline]
                fn sort(data: &mut [Self]) {
                    if cfg!(miri) {
                        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    } else {
                        unsafe { [<vqsort_ $t>](data.as_mut_ptr(), data.len()) };
                    }
                }

                #[inline]
                fn sort_descending(data: &mut [Self]) {
                    if cfg!(miri) {
                        data.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                    } else {
                        unsafe { [<vqsort_ $t _descending>](data.as_mut_ptr(), data.len()) };
                    }
                }
            }
        }
    )*)
}

vqsort_impl! { i16 u16 i32 u32 i64 u64 f32 f64 }

macro_rules! vqsort_u {
    ($($t:expr)*) => ($(
        paste::paste! {
            #[cfg(target_pointer_width = "" $t)]
            #[inline]
            fn sort(data: &mut [Self]) {
                if cfg!(miri) {
                    data.sort_unstable();
                } else {
                    unsafe { [<vqsort_u $t>](data.as_mut_ptr().cast(), data.len()) };
                }
            }

            #[cfg(target_pointer_width = "" $t)]
            #[inline]
            fn sort_descending(data: &mut [Self]) {
                if cfg!(miri) {
                    data.sort_unstable_by_key(|&x| core::cmp::Reverse(x));
                } else {
                    unsafe { [<vqsort_u $t _descending>](data.as_mut_ptr().cast(), data.len()) };
                }
            }
        }
    )*)
}

impl VqSort for usize {
    vqsort_u! { 16 32 64 }
}

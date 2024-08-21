pub trait TrySum<Item> {
    type Output;

    fn try_sum(self) -> Option<Self::Output>;
}

macro_rules! try_sum_impl {
    ($($t:ty)*) => {$(
        impl<I: Iterator<Item = $t>> TrySum<$t> for I {
            type Output = $t;

            #[inline]
            fn try_sum(mut self) -> Option<$t> {
                let zero: $t = 0;

                self.try_fold(zero, |a, v| a.checked_add(v))
            }
        }

        impl<'a, I: Iterator<Item = &'a $t>> TrySum<&'a $t> for I {
            type Output = $t;

            #[inline]
            fn try_sum(mut self) -> Option<$t> {
                let zero: $t = 0;

                self.try_fold(zero, |a, v| a.checked_add(*v))
            }
        }
    )*};
}

try_sum_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

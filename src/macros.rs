macro_rules! impl_into_u8 {
    ($t:ty) => {
        impl From<$t> for u8 {
            fn from(k: $t) -> u8 {
                k as u8
            }
        }
    };
}
pub(crate) use impl_into_u8;

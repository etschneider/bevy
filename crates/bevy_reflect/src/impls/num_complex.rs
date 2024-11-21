use crate as bevy_reflect;
use crate::{std_traits::ReflectDefault, ReflectDeserialize, ReflectSerialize};
use bevy_reflect_derive::impl_reflect;
use num_complex::Complex;

impl_reflect!(
    #[reflect(PartialEq, Serialize, Deserialize, Default)]
    #[type_path = "num_complex"]
    struct Complex<T: PartialEq + Default + serde::Serialize + for<'a> serde::Deserialize<'a>> {
        re: T,
        im: T,
    }
);

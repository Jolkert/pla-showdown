#![allow(dead_code, unused_imports)]
#![feature(let_chains)]

macro_rules! mod_pub_use_all {
    () => {};
    ($mod_name:ident $(,)?) => {
        mod $mod_name;
        pub use $mod_name::*;
    };
    ($($mod_name:ident),* $(,)?) => {
        $(
            mod $mod_name;
            pub use $mod_name::*;
        )*
    }
}

pub mod data;
pub mod parsing;

pub type BoxSlice<T> = Box<[T]>;
pub type BoxStr = Box<str>;

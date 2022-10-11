#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(clippy::field_reassign_with_default)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

#[macro_use]
extern crate log;

mod consts;
mod error;

pub mod prelude {
    pub use crate::consts::Consts;
    pub use crate::error::AuroraError;
}

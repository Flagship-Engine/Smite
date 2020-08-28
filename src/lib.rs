#[macro_use]
mod util;

pub mod angle;
#[rustfmt::skip]
pub mod traits;
pub mod types;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::{
        traits::float::*,
        traits::integer::*,
        traits::*
    };
}

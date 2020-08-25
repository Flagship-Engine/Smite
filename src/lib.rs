#[rustfmt::skip]
pub mod traits;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::{
        traits::float::*,
        traits::integer::*,
        traits::*
    };
}

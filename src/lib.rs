#[rustfmt::skip]
pub mod traits;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::{
        traits::*,
        traits::float::*,
        traits::generic::*,
        traits::integer::*,
    };
}

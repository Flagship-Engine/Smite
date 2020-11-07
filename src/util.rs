macro_rules! export {
    ($module:ident :: $Class:ident) => {
        mod $module;
        pub use $module::$Class;
    };
    ($module:ident :: { $($Class:ident),+ } ) => {
        mod $module;
        $(pub use $module::$Class;)*
    };
    ($($module:ident :: $Class:tt ;)*) => {
        $(export!($module :: $Class);)*
    };
}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {
        assert_approx_eq!($left, $right, "");
    };
    ($left:expr, $right:expr, $msg:expr) => {
        match (&$left, &$right, &$msg) {
            (left_val, right_val, msg) => {
                if left_val - 10.0 * core::f32::EPSILON > *right_val
                    || left_val + 10.0 * core::f32::EPSILON < *right_val
                {
                    panic!(
                        r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
                        &*left_val, &*right_val, &*msg
                    )
                }
            }
        }
    };
}

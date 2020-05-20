// $cargo expand --ugly
// $cargo clippy --all-targets --all-features -- -D warnings

mod r#enum;
mod r#struct;

#[macro_export]
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}

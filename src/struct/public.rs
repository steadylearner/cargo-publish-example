// https://doc.rust-lang.org/rustdoc/documentation-tests.html#documenting-macros

/// Similar to `private_struct!` but public.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate publish;
/// # fn main() {
/// public_struct!(
///     // pub is required before 'struct' when you use public_struct!
///     pub struct MessageBase {
///         pub text: String
///         // text: String // pub is optional in fields.
///     }
/// ); // It is lazy. Nothing is made yet.
///
/// MessageBase!(); // You have to call it to use the struct.
/// let message = MessageBase {
///     text: "First Message".into(),
/// };
/// println!("{}", &message.text);
/// # }
/// ```
#[macro_export]
macro_rules! public_struct {
    (pub struct $commonstruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    () => {
                        pub struct $commonstruct {
                            $( $commonfieldpub $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}

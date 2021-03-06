/// Use it to to create, extend and reuse fields from private struct definition.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate publish;
/// # fn main() {
/// private_struct!(
///     struct MessageBase {
///         text: String
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
macro_rules! private_struct {
    (struct $commonstruct:ident { $( $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    () => {
                        struct $commonstruct {
                            $( $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}

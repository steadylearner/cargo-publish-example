/// Use it to to create, extend and reuse fields in private enum.
///
/// # Example
/// 
/// ```
/// # #[macro_use] extern crate publish;
/// # fn main() {
/// private_enum!(
///     // pub is required before 'enum' when you use public_enum!
///     enum WebEventBase {
///         PageLoad,
///         PageUnload, // , here is required if you want to extend the fields later.
///     }
/// ); // It is lazy. Nothing is made yet.
///
/// WebEventBase!(); // You have to call it to use the enum.
///
/// fn inspect(event: WebEventBase) {
///     match event {
///         WebEventBase ::PageLoad => println!("page loaded"),
///         WebEventBase ::PageUnload => println!("page unloaded"),
///     }
/// }
///
/// let load    = WebEventBase::PageLoad;
/// let unload  = WebEventBase::PageUnload;
///
/// inspect(load);
/// inspect(unload);
/// # }
/// ```
#[macro_export]
macro_rules! private_enum {
    (enum $commonenum:ident { $( $commonfield:tt )+}) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    () => {
                        enum $commonenum {
                            $( $commonfield )+
                        }
                    };
                }
            }
        }
    };
}

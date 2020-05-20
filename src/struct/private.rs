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

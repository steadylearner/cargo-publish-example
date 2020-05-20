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

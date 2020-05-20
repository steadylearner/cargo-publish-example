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

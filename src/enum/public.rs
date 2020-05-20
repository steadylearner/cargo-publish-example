#[macro_export]
macro_rules! public_enum {
    (pub enum $commonenum:ident { $( $commonfield:tt )+}) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    () => {
                        pub enum $commonenum {
                            $( $commonfield )+
                        }
                    };
                }
            }
        }
    };
}


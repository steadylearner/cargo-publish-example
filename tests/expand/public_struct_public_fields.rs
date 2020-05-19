use publish::{
    nested_macro,
    public_struct,
};

pub fn main() {
    public_struct!(
        pub struct MessageBase {
            pub text: String
            pub read: bool,
        }
    );

    MessageBase!(); // You have to call it to use.
}
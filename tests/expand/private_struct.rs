use publish::{
    nested_macro,
    private_struct,
};

pub fn main() {
    private_struct!(
        struct MessageBase {
            text: String
        }
    );

    MessageBase!(); // You have to call it to use.
}


use publish::{
    nested_macro,
    private_enum,
};

pub fn main() {
    private_enum!(
        enum WebEventBase {
            PageLoad,
            PageUnload, // , here is required.
        }
    );

    WebEventBase!(); // You have to call it to use.
}



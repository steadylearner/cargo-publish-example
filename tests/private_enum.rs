use publish::{
    nested_macro,
    private_enum,
};

private_enum!(
    enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required.
    }
);

WebEventBase!(); // You have to call it to use.

fn inspect(event: WebEventBase) {
    match event {
        WebEventBase ::PageLoad => println!("page loaded"),
        WebEventBase ::PageUnload => println!("page unloaded"),
    }
}

// $cargo test -- --nocapture
#[test]
fn private_enum() {
    let load    = WebEventBase::PageLoad;
    let unload  = WebEventBase::PageUnload;

    inspect(load);
    inspect(unload);
}
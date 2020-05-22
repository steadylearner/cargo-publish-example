import subprocess as cmd

from watch import (
    manually_reload_the_doc,
    automatically_reload_the_doc,
)

package = "publish"  # It should be the same to what defined in Cargo.toml
response = input(
    "Cargo [f]ormat, [l]int with clippy, [t]est, [p]ublish or [d]ocumenation?\n")
    # "Cargo [f]ormat and fix, [l]int with clippy, [t]est, [p]ublish or [d]ocumenation?\n")

if response.startswith("f"):
    cp = cmd.run(f"cargo fmt", check=True, shell=True)
    # cp = cmd.run(f"cargo fix", check=True, shell=True)
elif response.startswith("l"):
    cp = cmd.run(
        f"cargo clippy --all-targets --all-features -- -D warnings", check=True, shell=True)
elif response.startswith("t"):
    cp = cmd.run(f"cargo test pass", check=True, shell=True)
elif response.startswith("p"):
    # You should commit first with $python commit.py
    cp = cmd.run(f"cargo verify-project", check=True, shell=True)
    # https://doc.rust-lang.org/cargo/reference/publishing.html#packaging-a-crate
    # login, token, publication etc should be manual.($cargo publish)

    response = input(
        "[t]est publication or [l]ist the created files for crates.io?\n")

    if response.startswith("t"):
        cp = cmd.run(
            f"cargo publish --dry-run", check=True, shell=True)
    else:
        cp = cmd.run(
            f"cargo package --list", check=True, shell=True)    
else: # Payload
    response = input(
        "[w]atch, [c]reate, [r]ead or [t]est the documentation?\n")

    if response.startswith("w"):
        response = input(
            "[a]utomatically reload or [m]anually?\n")
        if response.startswith("a"):
            # Use this when your machine is fast enough.
            automatically_reload_the_doc(package)
        else:
            manually_reload_the_doc(package)

    elif response.startswith("c"):
        cp = cmd.run(
            f"cargo doc --lib --open", check=True, shell=True)
    elif response.startswith("r"):
        cp = cmd.run(
            f"python -m webbrowser target/doc/{package}/index.html", check=True, shell=True)
    else:
        # Payload
        cp = cmd.run(
            f"cargo test --doc", check=True, shell=True)

# $cargo update to update dependencies.
# $cargo test -- --test-threads=1 to test a project that involves database mutation etc.

# $cargo verify-project
# {"success":"true"}
 
# Refer to them.
# https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

# https://github.com/steadylearner/Rust-Full-Stack/blob/master/dev.py
# https://doc.rust-lang.org/cargo/commands/cargo-doc.html

# [cargo fmt](https://github.com/rust-lang/rustfmt)
# [cargo clippy](https://github.com/rust-lang/rust-clippy)

# $rustdoc -h
# cp = cmd.run(
#     f"rustdoc src/lib.rs --crate-name {package} --output cargo/", check=True, shell=True)

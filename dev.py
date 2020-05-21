import subprocess as cmd
from watch import (
    browsersync, 
    # withoutbrowsersync
)

package = "publish" # It should be the same to what defined in Cargo.toml
response = input(
    "Cargo [f]ormat, [t]est, [d]ocumenation, [p]ublish or [l]int with clippy?\n")

if response.startswith("f"):
    cp = cmd.run(f"cargo fmt", check=True, shell=True)
elif response.startswith("t"):
    cp = cmd.run(f"cargo test pass", check=True, shell=True)
elif response.startswith("d"):
    response = input(
        "[w]atch, [c]reate, [r]ead or [t]est the documentation?\n")

    if response.startswith("w"):
        # withoutbrowsersync(package)

        browsersync(package)

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
elif response.startswith("p"):
    pass
else:
    cp = cmd.run(
        f"cargo clippy --all-targets --all-features -- -D warnings", check=True, shell=True)

# Refer to them.
# https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

# https://github.com/steadylearner/Rust-Full-Stack/blob/master/dev.py
# https://doc.rust-lang.org/cargo/commands/cargo-doc.html

# [cargo fmt](https://github.com/rust-lang/rustfmt)
# [cargo clippy](https://github.com/rust-lang/rust-clippy)

# $rustdoc -h
# cp = cmd.run(
#     f"rustdoc src/lib.rs --crate-name {package} --output cargo/", check=True, shell=True)

import subprocess as cmd

# response = input("Cargo [f]ormat, [t]est, [p]ublish or [l]int with clippy?\n")
response = input("Cargo [f]ormat, [t]est or [l]int with clippy?\n")

if response.startswith("f"):
    cp = cmd.run(f"cargo fmt", check=True, shell=True)
elif response.startswith("t"):
    cp = cmd.run(f"cargo test pass", check=True, shell=True)
# elif response.startswith("p"):
#     cp = cmd.run(f"cp target/debug/main main && ./main", check=True, shell=True)
else:
    cp = cmd.run(
        f"cargo clippy --all-targets --all-features -- -D warnings", check=True, shell=True)

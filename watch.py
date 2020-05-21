import subprocess as cmd


def manually_reload_the_doc(package: str):
    cp = cmd.run(
        f"python -m webbrowser target/doc/{package}/index.html", check=True, shell=True)

    cp = cmd.run(f"cargo watch -s 'cargo doc --lib'",
                 check=True, shell=True)


def automatically_reload_the_doc(package: str):
    # https://browsersync.io/($npm install -g browser-sync)
    # It take time to reload because Python, Rust and JavaScript are used at the same time.
    # https://benjamincongdon.me/blog/2018/08/22/Live-Refreshing-Cargo-Docs/
    # https://www.browsersync.io/docs/command-line

    # Wait and just referesh when first uploaded.
    cp = cmd.run(
        f"python -m webbrowser http://localhost:3000/{package}", check=True, shell=True)
    cp = cmd.run(
        f"cargo doc --lib && browser-sync start --server target/doc --files target/doc --no-open", check=True, shell=True)
    cp = cmd.run(
        f"cargo watch -s 'cargo doc --lib && browser-sync reload'", check=True, shell=True)

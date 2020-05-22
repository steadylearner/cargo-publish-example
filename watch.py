import subprocess as cmd
from threading import Timer

# It will be much faster to use this with ctrl+r normally
# than automatically_reload_the_doc.
def manually_reload_the_doc(package: str):
    cp = cmd.run(
        f"python -m webbrowser target/doc/{package}/index.html", check=True, shell=True)
    cp = cmd.run(f"cargo watch -s 'cargo doc --lib'",
                 check=True, shell=True)

# Function to use Timer API below.
def open_browser_sync_server(package: str):
    cp = cmd.run(
        f"python -m webbrowser http://localhost:3000/{package}", check=True, shell=True)

# You should have fast enough machine to run
# Rust, Python, JavaScript(TypeScript), VScode, Web browsers, CLI etc.
def automatically_reload_the_doc(package: str):
    # https://browsersync.io/($npm install -g browser-sync)
    # It take time to reload because Python, Rust and JavaScript are used at the same time.
    # https://benjamincongdon.me/blog/2018/08/22/Live-Refreshing-Cargo-Docs/
    # https://www.browsersync.io/docs/command-line

    # Wait before browser sync work.
    r = Timer(10.0, open_browser_sync_server, (package,))
    r.start()

    cp = cmd.run(
        f"cargo doc --lib && browser-sync start --server target/doc --files target/doc --no-open", check=True, shell=True)
    cp = cmd.run(
        f"cargo watch -s 'cargo doc --lib && browser-sync reload'", check=True, shell=True)

    # Shoud see similar to this.
    # [Browsersync] Reloading Browsers... (buffered 51 events).



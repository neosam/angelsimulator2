# Angel Simulator 2

Trying to rewrite the Engelsimulator.

## Build and run

I tested this project on MacOS and it worked out of the box.  I don't know which
packages are required on any Linux distributions, sorry.  I guess, it should work
on Windows.

```
# Build
cargo build
# Run
cargo run
```

## Web Build

It's possible to run it in the browser.  You need wasm-unknown-unkown target
build and wasm-bindgen-cli to generate the JavaScript bindings.

```
# Build
cargo build --target wasm32-unknown-unknown --no-default-features --features web
# Generate JS Bindings
wasm-bindgen --out-dir web --out-name engelsimulator2 --target web --no-typescript target/wasm32-unknown-unknown/debug/engelsimulator2.wasm
# Share the content of the web directory over http (e.g. with python)
cd web
python3 -m http.server
```

Now you can run the game by open http://localhost:8000 in your browser.

## Trouble shooting

### wasm-bindgen version conflict
The wasm-bindgen-cli must exactly match the wasm-bindgen crate which was used
to compile the project.  If you get such a version conflict error message, then
you need to reinstall wasm-bindgen-cli with the version it wants.

```
cargo install -f wasm-bindgen-cli --version whatever.it.wants
```


### bevy_log build issues with wasm build
If you experience issues during the wasm build in bevy_log regarding a Subscriber
trait, you need to downgrade tracing-wasm in the Cargo.lock:

```cargo update -p tracing-wasm --precise 0.2.0```


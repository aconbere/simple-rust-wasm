# A very simple rust WASM web setup

This is as simple a set of tools to get started with rust and wasm as I could manage.

## Build

Rust is written in `./src` (see `src/lib.rs`), but before being used in the browser we need to compile the rust into wasm, and then we use `wasm-bindgen` to build integration javascript files and typescript types. `wasm-bindgen` places the output files into `./static/pkg` which are loaded by `static/index.html`.

Before running

```bash
./scripts/build
```

## Run

I used [basic-http-server](https://github.com/brson/basic-http-server) to host the static files. You can install it with `cargo install basic-http-server` running `./scripts/web` will start up a server hosting the site.

```
./scripts/web
```

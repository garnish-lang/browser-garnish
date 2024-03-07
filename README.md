# Browser Garnish
WebAssembly module used by Garnish core projects. 
Currently not intended for general use, so it is not published anywhere.
But can be pulled manually to be package in a site.

## Requirements

[Rust](https://www.rust-lang.org/) - Can be installed with [Rust Up](https://rustup.rs/). Will also install `cargo`.

[Wasm Pack](https://rustwasm.github.io/wasm-pack/) - Installed with `cargo install wasm`. Other methods on their site.

## Building
Use wasm-pack to build to `pkg` directory.

```shell
wasm-pack build
```

## Local Development
Test rust code
```shell
cargo test
```

Run simple testing site
```shell
cd site
npm run dev
```

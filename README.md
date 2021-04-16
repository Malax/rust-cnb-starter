# rust-cnb-starter
A template project for [Cloud Native Buildpacks](https://buildpacks.io/) written in the Rust programming language.

It targets `x86_64-unknown-linux-musl` as the platform for the buildpack and comes with tooling to support 
cross-compilation on macOS. It uses [libcnb](https://github.com/malax/libcnb) as the language binding for buildpacks.

## Requirements

- [rustup](https://rustup.rs/)
- `rustup target add x86_64-unknown-linux-musl`
- [cargo-make](https://github.com/sagiegurari/cargo-make): `cargo install cargo-make`

### macOS
- [homebrew-musl-cross](https://github.com/FiloSottile/homebrew-musl-cross): `brew install FiloSottile/musl-cross/musl-cross`

### Ubuntu

```
$ sudo apt install musl-tools
```

## Usage
Copy this repository and make the necessary changes for your buildpack. To build and run the buildpack:

```shell
cargo make development
pack build rust-cnb-starter -b target/buildpack --path $APP_DIR
```

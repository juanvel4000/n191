# ![n191 logo](assets/n191.svg) n191
## Fork of [microsoft/edit](https://github.com/microsoft/edit)

A fork of the simple editor for simple needs.

This is a fork of [microsoft/edit](https://github.com/microsoft/edit) which itself pays homage to [MS-DOS Editor](https://en.wikipedia.org/wiki/MS-DOS_Editor), but with a modern interface and input controls similar to  VS Code. The goal is to provide an accessible editor for users with minimal experience using terminals.

![Screenshot of Edit with the About dialog in the foreground](./assets/edit_hero_image.png)

## Installation
Coming soon...


## Build Instructions


* [Install Rust](https://www.rust-lang.org/tools/install)
* Install the nightly toolchain: `rustup install nightly`
  * Alternatively, set the environment variable `RUSTC_BOOTSTRAP=1`
* Clone the repository
* For a release build, run: `cargo build --config .cargo/release.toml --release`

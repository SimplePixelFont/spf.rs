[![Build](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/build.png)](https://github.com/SimplePixelFont/spf.rs/actions?query=workflow:"rust")
![Compatibility](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/compatibility.png)
![Tests](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/tests.png)
[![Coverage](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/coverage.png)](https://codecov.io/gh/SimplePixelFont/spf.rs)
[![Documentation](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/documentation.png)](https://gist.github.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4#file-documentation-md)
![Grammer](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/grammer.png)
![Lint](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/lint.png)
![MSRV](https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/msrv.png)

Parser library for the [SimplePixelFont file specifications](https://github.com/SimplePixelFont/Specification), written in Rust.
`spf.rs` is both a native crate and also an FFI library which can be used  in a variety of other programming languages which support library loading.

### Installation

- To install `spf.rs` as a rust crate run the following command in your cargo project or [read more](https://docs.rs/spf/latest/spf/articles/installing/index.html#installing-with-cargo-and-rust):
```sh
cargo add spf
```

- To use `spf.rs` as an FFI library in your language of choice you must first download a pre-built library artifact of `spf.rs` from the [releases section](https://github.com/SimplePixelFont/spf.rs/releases) which includes the dynamic library along with a header file. Pre-built artifacts are available for many architectures, however if there is no build for your architecture-including embedded devices-you can [compile `spf.rs` from source](https://docs.rs/spf/latest/spf/articles/installing/index.html#compiling-spfrs-from-source).

### Usage

**Note:** `spf.rs` documentation is currently out of date, however actively being updated to reflect the SimplePixelFont new standardized specification.

Usage varies depending on the programming language you choose. For a guide using the native Rust interface check out the [Getting Started in Rust](https://docs.rs/spf/latest/spf/articles/getting_started/index.html) article. You can also check out the [Using the FFI in C](https://docs.rs/spf/latest/spf/articles/c_usage/index.html) article for usage with the `spf.rs` library.

### Supported SPF Tables

`spf.rs` is the official parser for the SimplePixelFont file specifications, and will always attempt
to parallel developments within the specifictaitons. The following tables are supported:

| Type | Stability | Notes |
| ---- | --------- | ----- |
| Character Table | ✔ | `Added in v0.7.0-alpha.0` |
| Pixmap Table | ✔ | `Added in v0.7.0-alpha.0` |
| Color Table | ✔ | `Added in v0.7.0-alpha.0` |

Key:
- `⚠️` = Work in progress
- `❌` = Not implemented
- `✔` = Stable

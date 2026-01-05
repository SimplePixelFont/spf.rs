# Install `spf` crate with Cargo and Rust

To add `spf.rs` to your rust project, run the following command:
```bash
cargo add spf
```

Additionally, `spf.rs` includes modules which are enabled by default with the corrosponding features.

* ergonomics: [`crate::ergonomics`]
* ffi: [`crate::ffi`]
* tagging: [`crate::tagging`]
* afticles: [`crate::articles`]

And a few extra features for convienience and integration, also enabled by default.

* log: Integrate with Rust's log ecosysten.
* serde: Integrate with Rust's serde (serialization and deserialization) ecosystem.
* std: Add dependency on Rust's std crate which handles heap allocations automatically. You can disable this feature to allow a custom heap allocator, and use spf.rs on low-level and embeded hardware.

You can choose which features to use by editing the `Cargo.toml` file under the depenencies section, such as.
```toml
[dependencies]
spf = { version = "0.7.2", default-features = false, features = ["ffi", "std"]}
```

# Compile `spf.rs` library from source

Compiling the `spf.rs` library by hand is useful if you want to create a custom version of the library, and enable only modules your projects need. This can decrease the resulting binary size, and allow you to target platforms which don't have pre-built artifacts in the releases section. To compile `spf.rs` from source you will need the following prerequisites.

* [Rust Programming Language](https://rust-lang.org/tools/install/)
* [Git](https://git-scm.com/install/) (Optional)

Begin by cloning the repository with the following command:
```bash
git clone
# Alternavtivly, download the spf.rs repository and "cd" into the downloaded directory.
# cd downloads/spf.rs
```
Now run the following command.
```bash
# "--no-default-features" removes all default spf features from the current build as currently spf only compiles with features that do not require crate dependencies.
# "--features" specifies a list of features to compile the library with, currently the following work: "std", "ergonomics", "ffi", "tagging", and "articles".
# "--crate-type cdylib" produces a dynamic library which can be loaded at runtime.
# "--crate-type staticlib" produces a static library which can be linked at compile time.
# "target-feature=-crt-static" is used to fix static linking errors occuring in builds on some architectures. "cargo rustc" is also used in order to pass this Compiler flag
cargo rustc --release --no-default-features --features "ffi,std" -- --crate-type cdylib --crate-type staticlib -C target-feature=-crt-static
```
In `./target/release/` you should find a `spf.dll`, `libspf.so`, etc. (depending on your OS). This library can now be used in your programming language of choice.

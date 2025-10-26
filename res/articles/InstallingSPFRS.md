# Install with Cargo and Rust

To add `spf.rs` to your rust project run the following command:
```sh
cargo add spf
```
Additionally, `spf.rs` includes modules which are enabled by default with the corrosponding features.
* ergonomics: [`crate::ergonomics`]
* [`crate::ffi`] ("ffi")
* [`crate::articles`] ("articles").
And a few extra features that provide functionality in Rust.
* ""

You can choose which features to use by editing the `Cargo.toml` file under the depenencies section:
```toml
# Example
[dependencies]
spf = { version = "0.4", default-features = false, features = ["cache"]}
```

# Compiling `spf.rs` from source

Sometimes you may wish to compile the `spf.rs` library by hand. This may be useful especially if you wish to create a custom version of the library, using only modules your projects need and thus decreasing the resulting binary size. For this you will need the following:
    - Rust Programming Language
    - Git (Optional)

To begin we will first clone the repository with the following command:
```sh
git clone
# If you chose to not use Git, you can also download the spf.rs repo and cd into the downloaded directory, ex.
cd downloads/spf.rs
```
Now you can run:
```sh
cargo build --release
```
And if you check `./target/release/` you should find a `spf.dll`, `libspf.so`, etc. (depending on your OS). Now you can use this library in your programming language of choice.

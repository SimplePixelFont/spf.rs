# Build for MacOS targets
run(`sh -c "BINARYBUILDER_AUTOMATIC_APPLE=true julia -- ./.ci/build_tarballs.jl \"x86_64-apple-darwin\", \"aarch64-apple-darwin\""`)

# Build for Linux targets
run(`sh -c "julia -- ./.ci/build_tarballs.jl i686-linux-gnu, x86_64-linux-gnu, aarch64-linux-gnu, armv6l-linux-gnueabihf, armv7l-linux-gnueabihf, powerpc64le-linux-gnu, i686-linux-musl, x86_64-linux-musl, aarch64-linux-musl, armv6l-linux-musleabihf, armv7l-linux-musleabihf, x86_64-unknown-freebsd"`)

# Build for windows-gnu target
run(`sh -c "julia -- ./.ci/build_tarballs.jl x86_64-w64-mingw32"`)

# Build for windows-msvc target
run(`sh -c "cargo install --locked cargo-xwin"`)
run(`sh -c "rustup target add x86_64-pc-windows-msvc"`)
run(`sh -c "cargo xwin build --target x86_64-pc-windows-msvc --release"`)

# Build for WASM target
run(`sh -c "rustup target add wasm32-unknown-unknown"`)
run(`sh -c "cargo build --target wasm32-unknown-unknown --release"`)

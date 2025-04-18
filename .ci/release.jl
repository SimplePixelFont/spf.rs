### BinaryBuilder Builds ###

jl_platforms = [
    "'i686-linux-gnu'"
    #"'x86_64-linux-gnu'"
    #"'aarch64-linux-gnu'"
    #"'armv6l-linux-gnueabihf'"
    #"'armv7l-linux-gnueabihf'"
    #"'powerpc64le-linux-gnu'"
    # "'riscv64-linux-gnu'" no rust toolchain for this platform
    # "'i686-linux-musl'"
    # "'x86_64-linux-musl'"
    # "'aarch64-linux-musl'"
    # "'armv6l-linux-musleabihf'"
    # "'armv7l-linux-musleabihf'"
    # "'x86_64-apple-darwin'"
    # "'aarch64-apple-darwin'"
    # "'x86_64-unknown-freebsd'"
    # "'aarch64-unknown-freebsd'" no rust toolchain for this platform
    # "'i686-w64-mingw32'" fails to build on this platform
    "'x86_64-w64-mingw32'"
]

# Github Actions runs out of disk space: The solution is to delete all artifacts after each platform build.
for platform in jl_platforms
    run(`sh -c "BINARYBUILDER_RUNNER='privileged' BINARYBUILDER_AUTOMATIC_APPLE=true julia -- ./.ci/build_tarballs.jl $platform"`)
    run(`sh -c "sudo rm -rf '/home/runner/.julia/artifacts/'"`)
end

### Extra Platform Builds ###

# Windows MSVC build
run(`sh -c "cargo install --locked cargo-xwin"`)
run(`sh -c "rustup target add x86_64-pc-windows-msvc"`)
run(`sh -c "cargo xwin build --target x86_64-pc-windows-msvc --release"`)

# WASM build
run(`sh -c "rustup target add wasm32-unknown-unknown"`)
run(`sh -c "cargo build --target wasm32-unknown-unknown --release"`)

### Header Files Generation ###
run(`sh -c "cargo install cbindgen"`)
run(`sh -c "cbindgen --output bindspf.h --lang c --cpp-compat"`)

### Bring Builds Together ###
mkdir("artifacts")
for platform in jl_platforms
    mv("products/spf.v0.5.0.$platform.tar.gz", "artifacts/spf.v0.5.0.$platform.tar.gz")
end

using Pkg
Pkg.add("Tar")
using Tar

mkdir("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-mingw32")
mv("target/x86_64-pc-windows-msvc/release/spf.dll", "target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-mingw32/spf.dll")
create("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-mingw32", "artifacts/spf.v0.5.0.x86_64-w64-mingw32.tar.gz")

mkdir("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown")
mv("target/wasm32-unknown-unknown/release/spf.wasm", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/spf.wasm")
create("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown", "artifacts/spf.v0.5.0.wasm32-unknown-unknown.tar.gz")

mkdir("headers")
mv("bindspf.h", "headers/bindspf.h")
create("headers", "artifacts/headers.tar.gz")

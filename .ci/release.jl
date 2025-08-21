### Header Files Generation (used by BinaryBuilder) ###
run(`sh -c "cargo install cbindgen"`)
run(`sh -c "cbindgen --output target/spf.h --lang c --cpp-compat"`)

### BinaryBuilder Builds ###

jl_platforms = [
    "'i686-linux-gnu'"
    "'x86_64-linux-gnu'"
    "'aarch64-linux-gnu'"
    "'armv6l-linux-gnueabihf'"
    "'armv7l-linux-gnueabihf'"
    "'powerpc64le-linux-gnu'"
    # "'riscv64-linux-gnu'" No Rust toolchain for this platform
    "'i686-linux-musl'"
    "'x86_64-linux-musl'"
    "'aarch64-linux-musl'"
    "'armv6l-linux-musleabihf'"
    "'armv7l-linux-musleabihf'"
    "'x86_64-apple-darwin'"
    "'aarch64-apple-darwin'"
    "'x86_64-unknown-freebsd'"
    # "'aarch64-unknown-freebsd'" No Rust toolchain for this platform
    # "'i686-w64-mingw32'" Fails to build on this platform
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

# Will be figured out during 0.6.x
# WASM build
# run(`sh -c "rustup target add wasm32-unknown-unknown"`)
# run(`sh -c "cargo build --target wasm32-unknown-unknown --release"`)

# Will be figured out during 0.6.x
### WASM-Bindgen Generation ###
# run(`sh -c "cargo install wasm-bindgen-cli"`)
# run(`sh -c "wasm-bindgen --out-dir target/wasm32-unknown-unknown/dist target/wasm32-unknown-unknown/release/spf.wasm"`)

### Bring Builds Together ###
mkdir("artifacts")
for platform in jl_platforms
    platform = platform[2:end-1]
    mv("products/spf.v0.5.0.$platform.tar.gz", "artifacts/spf.v0.5.0.$platform.tar.gz")
end

using Pkg
Pkg.add("Tar")
using Tar

mkdir("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc")
mkpath("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/lib")
mkpath("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/include")
mkpath("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/share/licenses/spf")

mv("target/x86_64-pc-windows-msvc/release/spf.dll", "target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/lib/spf.dll")
cp("target/spf.h", "target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/include/spf.h")
cp("LICENSE-APACHE", "target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc/share/licenses/spf/LICENSE-APACHE")

Tar.create("target/x86_64-pc-windows-msvc/release/spf.v0.5.0.x86_64-w64-msvc", "artifacts/spf.v0.5.0.x86_64-w64-msvc.tar.gz")

# Will be figured out during 0.6.x
# mkdir("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown")
# mkpath("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/lib")
# mkpath("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/include")
# mkpath("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/share/licenses/spf")

# mv("target/wasm32-unknown-unknown/release/spf.wasm", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/lib/spf_pure.wasm")
# mv("target/wasm32-unknown-unknown/dist/spf_bg.wasm", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/lib/spf_bg.wasm")
# mv("target/wasm32-unknown-unknown/dist/spf.js", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/include/spf.js")
# mv("target/wasm32-unknown-unknown/dist/spf_bg.js", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/include/spf_bg.js")
# cp("LICENSE-APACHE", "target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown/share/licenses/spf/LICENSE-APACHE")

# Tar.create("target/wasm32-unknown-unknown/release/spf.v0.5.0.wasm32-unknown-unknown", "artifacts/spf.v0.5.0.wasm32-unknown-unknown.tar.gz")

### BinaryBuilder Builds ###

jl_platforms = [
    "'i686-linux-gnu'"
    "'x86_64-linux-gnu'"
    "'aarch64-linux-gnu'"
    "'armv6l-linux-gnueabihf'"
    "'armv7l-linux-gnueabihf'"
    "'powerpc64le-linux-gnu'"
    # "'riscv64-linux-gnu'" no rust toolchain for this platform
    "'i686-linux-musl'"
    "'x86_64-linux-musl'"
    "'aarch64-linux-musl'"
    "'armv6l-linux-musleabihf'"
    "'armv7l-linux-musleabihf'"
    "'x86_64-apple-darwin'"
    "'aarch64-apple-darwin'"
    "'x86_64-unknown-freebsd'"
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
run(`sh -c "cbindgen --output bindspf.h --lang c --cpp-compat`)

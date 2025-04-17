# Note that this script can accept some limited command-line arguments, run
# `julia build_tarballs.jl --help` to see a usage message.
using Pkg
try
    using BinaryBuilder
catch
    Pkg.add("BinaryBuilder")
    using BinaryBuilder
end

name = "spf"
version = v"0.5.0"

# Collection of sources required to complete build
sources = [
    GitSource("https://github.com/SimplePixelFont/spf.rs", "6009a2b58281ba30abbf025d4a9f1f81955006e2")
    # ArchiveSource("https://github.com/SimplePixelFont/spf.rs/archive/refs/tags/v$(version).tar.gz", "70fc93369d804a92da3cd727d3c482de8e97d893990f243a0ff3533bc64754a1")
]

# Bash recipe for building across all platforms
script = raw"""
cd $WORKSPACE/srcdir
cd spf.rs
mkdir target
RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
ls target/${rust_target}/release

if [[ "${rust_target}" == "x86_64-pc-windows-gnu" ]]; then
    install -D -m 755 "target/${rust_target}/release/spf.${dlext}" "${libdir}/libspf.${dlext}"
else
    install -D -m 755 "target/${rust_target}/release/libspf.${dlext}" "${libdir}/libspf.${dlext}"
fi

install_license UNLICENSE
"""

# These are the platforms we will build for by default, unless further
# platforms are passed in on the command line
platforms = [
    Platform("armv7l", "linux"; call_abi = "eabihf", libc = "glibc"),
    Platform("armv7l", "linux"; call_abi = "eabihf", libc = "musl"),
    #Platform("i686", "linux"; libc = "musl"), fails for some reason
    Platform("i686", "linux"; libc = "glibc"),
    Platform("armv6l", "linux"; call_abi = "eabihf", libc = "glibc"),
    Platform("powerpc64le", "linux"; libc = "glibc"),
    Platform("x86_64", "macos"; ),
    Platform("x86_64", "linux"; libc = "glibc"),
    Platform("aarch64", "linux"; libc = "musl"),
    Platform("armv6l", "linux"; call_abi = "eabihf", libc = "musl"),
    #Platform("x86_64", "linux"; libc = "musl"),
    Platform("x86_64", "freebsd"; ),
    Platform("x86_64", "windows"; ),
    Platform("aarch64", "macos"; ),
    Platform("aarch64", "linux"; libc = "glibc")
]


# The products that we will ensure are always built
products = [
    LibraryProduct("libspf", :libspf)
]

# Dependencies that must be installed before this package can be built
dependencies = Dependency[
]

ENV["BINARYBUILDER_AUTOMATIC_APPLE"] = true

# Build the tarballs, and possibly a `build.jl` as well.
build_tarballs(ARGS, name, version, sources, script, platforms, products, dependencies; julia_compat="1.6", compilers = [:rust, :c])

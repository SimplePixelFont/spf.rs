
# Build for MacOS targets
#run(`sh -c "BINARYBUILDER_AUTOMATIC_APPLE=true julia -- ./.ci/build_tarballs.jl 'x86_64-apple-darwin','aarch64-apple-darwin'"`)

jl_platforms = [
    "'i686-linux-gnu'"
    "'x86_64-linux-gnu'"
    "'armv6l-linux-gnueabihf'"
    "'armv7l-linux-gnueabihf'"
    "'powerpc64le-linux-gnu'"
    "'i686-linux-musl'"
    "'x86_64-linux-musl'"
    "'aarch64-linux-musl'"
    "'armv6l-linux-musleabihf'"
    "'armv7l-linux-musleabihf'"
    "'x86_64-unknown-freebsd'"
]

for platform in jl_platforms
    run(`sh -c "BINARYBUILDER_RUNNER='privileged' julia -- ./.ci/build_tarballs.jl $platform"`)
    run(`sh -c "sudo rm -rf '/home/runner/.julia/artifacts/'"`)
end


# Build for Linux GNU targets
#run(`sh -c "julia -- ./.ci/build_tarballs.jl 'i686-linux-gnu','x86_64-linux-gnu','aarch64-linux-gnu'"`)

# Build for Linux MUSL targets
#run(`sh -c "julia -- ./.ci/build_tarballs.jl 'i686-linux-musl','x86_64-linux-musl','aarch64-linux-musl'"`)

# Build for extra Linux targets
#run(`sh -c "julia -- ./.ci/build_tarballs.jl 'armv6l-linux-gnueabihf','armv7l-linux-gnueabihf','armv6l-linux-musleabihf','armv7l-linux-musleabihf','x86_64-unknown-freebsd'"`)

# Build for Windows targets
#run(`sh -c "julia -- ./.ci/build_tarballs.jl 'x86_64-w64-mingw32'"`)

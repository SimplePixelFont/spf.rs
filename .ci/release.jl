
# Build for MacOS targets
cmd = `sh -c "BINARYBUILDER_AUTOMATIC_APPLE=true julia -- ./.ci/build_tarballs.jl 'x86_64-apple-darwin'"`

println("Running command: $cmd")
run(cmd)

# Build for Linux targets
run(`$julia -- build_tarballs.jl "i686-linux-gnu", "x86_64-linux-gnu", "aarch64-linux-gnu", "armv6l-linux-gnueabihf", "armv7l-linux-gnueabihf", "powerpc64le-linux-gnu", "i686-linux-musl", "x86_64-linux-musl", "aarch64-linux-musl", "armv6l-linux-musleabihf", "armv7l-linux-musleabihf", "x86_64-unknown-freebsd"`)

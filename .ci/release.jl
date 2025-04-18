
println(readdir())
cd(".ci")
println(readdir())
println(Base.julia_cmd())
println("\n\n")
# Build for MacOS targets
julia = "/home/runner/.julia/juliaup/julia-1.7.0+0.x64.linux.gnu/bin/julia"
cmd = `sh -c "BINARYBUILDER_AUTOMATIC_APPLE=true julia -- ./.ci/build_tarballs.jl x86_64-apple-darwin, aarch64-apple-darwin"`

println("Running command: $cmd")
run(cmd)

# Build for Linux targets
run(`sh -c "julia -- ./.ci/build_tarballs.jl i686-linux-gnu, x86_64-linux-gnu, aarch64-linux-gnu, armv6l-linux-gnueabihf, armv7l-linux-gnueabihf, powerpc64le-linux-gnu, i686-linux-musl, x86_64-linux-musl, aarch64-linux-musl, armv6l-linux-musleabihf, armv7l-linux-musleabihf, x86_64-unknown-freebsd"`)

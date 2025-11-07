using Pkg
try 
    using Colors 
catch
    println("Colors.jl not found. Installing...")
    Pkg.add("Colors")
    using Colors
end

if length(ARGS) < 3
    println("Usage: interpolate_colors.jl <start_color> <end_color> <percent>")
    println("Example: interpolate_colors.jl '#d94a69' #00bfa3' 50.0")
    exit(1)
end 

start_color = ARGS[1]
end_color = ARGS[2]
percent = ARGS[3]

start_rgb = parse(RGB, start_color)
end_rgb = parse(RGB, end_color)

fraction = parse(Float64, percent) / 100.0
r = start_rgb.r * (1 - fraction) + end_rgb.r * fraction
g = start_rgb.g * (1 - fraction) + end_rgb.g * fraction
b = start_rgb.b * (1 - fraction) + end_rgb.b * fraction

interpolated_color = hex(RGB(r, g, b))
println("COLOR='#$interpolated_color'")

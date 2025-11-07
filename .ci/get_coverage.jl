open("report.txt", "r") do file
    coverage_line = last(eachline(file))
    coverage = String(split(coverage_line, "%")[1])
    println("COVERAGE=$coverage")
end

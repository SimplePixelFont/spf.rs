open("report.txt", "r") do file
    coverage_line = last(eachline(file))
    coverage = String(split(coverage_line, "%")[1])
    open(ENV["GITHUB_ENV"], "a") do env_file
        println(env_file, "COVERAGE=$coverage")
    end
end

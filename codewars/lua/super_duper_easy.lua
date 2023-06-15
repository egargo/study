-- https://www.codewars.com/kata/55a5bfaa756cfede78000026/train/lua


function problem(a)
    if (type(a) == "string") then
        return "Error"
    end

    return a * 50 + 6;
end

print(problem(12))


return problem

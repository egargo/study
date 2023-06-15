-- https://www.codewars.com/kata/559a28007caad2ac4e000083/train/lua


local solution = {}

function solution.perimeter(n)
    local ft, st, nt = 1, 1, 0;
    local s = ft + st;

    for _ = 2,n,1 do
        nt = ft + st;
        st = ft;
        ft = nt;
        s = s + nt;
    end

    return s * 4

end

local i = io.read("*n")

print(perimeter(i))

return perimeter

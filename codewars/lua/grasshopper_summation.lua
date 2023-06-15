-- https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/lua


return function(n)
    local sum = 0;

    for i = 1, n, 1 do
        sum = sum + i
    end

    return sum
end

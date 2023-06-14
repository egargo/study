-- https://www.codewars.com/kata/52ceafd1f235ce81aa00073a/train/lua


function isPlural(n)
    local plural = false

    if (n < 1 or n> 1) then
        plural = true
    end

    return plural
end

print(isPlural(1))

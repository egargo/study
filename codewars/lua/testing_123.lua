-- https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/lua


function number(lines)
    local n = {}

    for i = 1, #(lines), 1 do
        print(lines[i])
        n[i] = tostring(i .. ": " .. lines[i])
    end

    return n
end

print(number({"a", "b", "c"}))

return number

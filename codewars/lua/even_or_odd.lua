-- https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/lua

local kata = {}

function kata.even_or_odd(number)
    if number % 2 == 0 then
        return "Even"
    else
        return "Odd"
    end
end

for i = 1, 10, 1 do
    print(i, kata.even_or_odd(i))
end


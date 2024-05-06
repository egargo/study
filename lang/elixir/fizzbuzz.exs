defmodule FizzBuzz do
    def fizzbuzz(n) when n > 0 do
        Enum.each(1..n, fn(n) ->
            cond do
                rem(n, 3) == 0 and rem(n, 5) == 0 -> IO.puts("FizzBuzz")
                rem(n, 3) == 0 -> IO.puts("Fizz")
                rem(n, 5) == 0 -> IO.puts("Buzz")
                true -> IO.puts(n)
            end
        end)
    end
end

FizzBuzz.fizzbuzz(20)

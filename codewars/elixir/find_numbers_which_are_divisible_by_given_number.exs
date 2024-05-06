# https://www.codewars.com/kata/55edaba99da3a9c84000003b/train/elixir

defmodule Filtron do
  def divisible_by(numbers, divisor) do
    Enum.filter(numbers, fn(n) ->
      rem(n, divisor) == 0
    end)
  end
end

IO.inspect(Filtron.divisible_by([1, 2, 3, 4, 5, 6], 2))

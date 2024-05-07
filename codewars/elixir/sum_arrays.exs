# https://www.codewars.com/kata/53dc54212259ed3d4f00071c/train/elixir

defmodule SumNumbers do
  def sum(numbers) do
    Enum.sum(numbers)
  end
end

IO.puts(SumNumbers.sum([1, 5.2, 4, 0, - 1]))

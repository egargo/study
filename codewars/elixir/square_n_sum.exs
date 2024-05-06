# https://www.codewars.com/kata/515e271a311df0350d00000f/train/elixir

defmodule SquareSum do
  def square_sum(nums) do
    Enum.sum(Enum.map(nums, fn(n) ->
      n ** 2
    end))
  end
end

SquareSum.square_sum([1, 2, 2])

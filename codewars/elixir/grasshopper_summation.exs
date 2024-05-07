# https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/elixir

defmodule Series do
  def summation(n) do
    Enum.sum(1..n)
  end
end

IO.inspect(Series.summation(1))
IO.inspect(Series.summation(2))
IO.inspect(Series.summation(8))
IO.inspect(Series.summation(12))

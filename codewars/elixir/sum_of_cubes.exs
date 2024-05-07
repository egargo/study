# https://www.codewars.com/kata/59a8570b570190d313000037/train/elixir

defmodule SumCubes do
  def sum_cubes(n) do
    1..n
      |> Enum.map(fn(i) -> i ** 3 end)
      |> Enum.sum()
  end
end

IO.inspect(SumCubes.sum_cubes(2))
IO.inspect(SumCubes.sum_cubes(3))
IO.inspect(SumCubes.sum_cubes(12))

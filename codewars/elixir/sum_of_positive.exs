# https://www.codewars.com/kata/5715eaedb436cf5606000381/train/elixir

defmodule Solution do
  def positive_sum(arr) do
    arr
      |> Enum.filter(fn(i) -> i >= 0 end)
      |> Enum.sum()
  end
end

IO.puts(Solution.positive_sum([1, 2, 2, 3, 4]))
IO.puts(Solution.positive_sum([]))

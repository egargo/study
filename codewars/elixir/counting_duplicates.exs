# https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/elixir

defmodule DuplicateCount do
  def count(str) do
    str
      |> String.downcase()
      |> String.graphemes()
      |> Enum.frequencies()
      |> Enum.map(fn({_k, v}) -> if v > 1, do: 1 , else: 0 end)
      |> Enum.sum()
  end
end

IO.puts(DuplicateCount.count(""))
IO.puts(DuplicateCount.count("abcde"))
IO.puts(DuplicateCount.count("aabbcde"))
IO.puts(DuplicateCount.count("aabBcde"))
IO.puts(DuplicateCount.count("Indivisibility"))
IO.puts(DuplicateCount.count("Indivisibilities"))

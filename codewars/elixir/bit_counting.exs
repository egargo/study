# https://www.codewars.com/kata/526571aae218b8ee490006f4/train/elixir

defmodule CountBits do
  def bits(n) do
    n
      |> Integer.to_string(2)
      |> String.graphemes()
      |> Enum.filter(fn(b) ->  b == "1" end)
      |> Enum.count()
  end
end

IO.puts(CountBits.bits(0))
IO.puts(CountBits.bits(4))
IO.puts(CountBits.bits(7))
IO.puts(CountBits.bits(9))
IO.puts(CountBits.bits(10))
IO.puts(CountBits.bits(1234))

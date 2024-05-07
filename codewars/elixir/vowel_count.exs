# https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/elixir

defmodule VowelCount do
  def get_count(str) do
    str
      |> String.graphemes()
      |> Enum.map(fn(c) ->
          case Enum.member?(["a", "e", "i", "o", "u"], c) do
            true -> 1
            _ -> 0
          end
        end)
      |> Enum.sum()
  end
end

IO.inspect(VowelCount.get_count("abracadabra"))

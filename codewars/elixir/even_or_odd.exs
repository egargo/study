# https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/elixir

defmodule Codewars do
  def even_or_odd(number) do
    case rem(number, 2) == 0 do
      true -> "Even"
      _ -> "Odd"
    end
  end
end

IO.puts(Codewars.even_or_odd(12))
IO.puts(Codewars.even_or_odd(1))

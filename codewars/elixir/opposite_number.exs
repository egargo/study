# https://www.codewars.com/kata/56dec885c54a926dcd001095/train/elixir

defmodule Opposite do
  def opposite(number) do
    number * -1
  end
end

IO.puts(Opposite.opposite(-12))

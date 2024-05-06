# https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/elixir

defmodule Inverter do
  def invert(list) do
    Enum.map(list, fn(n) ->
      n * (-1)
    end)
  end
end

IO.inspect(Inverter.invert([1, 2, 3, 4, 5]))

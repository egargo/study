# https://www.codewars.com/kata/544675c6f971f7399a000e79/train/elixir

defmodule Numerify do
  def string_to_number(str) do
    String.to_integer(str)
  end
end

IO.puts(Numerify.string_to_number("12"))

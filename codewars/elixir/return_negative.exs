defmodule Negator do
  def make_negative(num) do
    if num < 0 do
      num
    else
      num * - 1
    end
  end
end

IO.puts(Negator.make_negative(2))
IO.puts(Negator.make_negative(12))
IO.puts(Negator.make_negative(22))
IO.puts(Negator.make_negative(0))
IO.puts(Negator.make_negative(-2))
IO.puts(Negator.make_negative(-12))
IO.puts(Negator.make_negative(-22))
IO.puts(Negator.make_negative(-1))

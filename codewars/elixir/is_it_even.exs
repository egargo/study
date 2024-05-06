defmodule Evenator do
  def even?(n) do
    cond do
      is_integer(n) and rem(n, 2) == 0 -> true
      true -> false
    end
  end
end

IO.puts(Evenator.even?(0))
IO.puts(Evenator.even?(0.5))
IO.puts(Evenator.even?(1))
IO.puts(Evenator.even?(2))
IO.puts(Evenator.even?(-4))

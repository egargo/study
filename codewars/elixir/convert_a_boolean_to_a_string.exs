# https://www.codewars.com/kata/551b4501ac0447318f0009cd/train/elixir

defmodule Kata do
    def boolean_to_string(b) do
      "#{b}"
    end
end

IO.puts(Kata.boolean_to_string(true))
IO.puts(Kata.boolean_to_string(false))

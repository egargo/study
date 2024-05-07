# https://www.codewars.com/kata/57a0885cbb9944e24c00008e/train/elixir

defmodule Codewars do
  def remove_exclamation_marks(s) do
    String.replace(s, "!", "")
  end
end

IO.puts(Codewars.remove_exclamation_marks("Hello World!!!"))

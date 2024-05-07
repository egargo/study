# https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/elixir

defmodule Solution do
  def reverse(string) do
    String.reverse(string)
  end
end

IO.puts(Solution.reverse("eeb"))
IO.puts(Solution.reverse("bee"))

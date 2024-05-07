# https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/elixir

defmodule Solution do
  def repeat_str(n, s) do
    1..n
      |> Enum.map(fn(_) -> s end)
      |> Enum.join()
  end
end

IO.inspect(Solution.repeat_str(12, "Bee"))

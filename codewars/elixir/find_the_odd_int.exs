# https://www.codewars.com/kata/54da5a58ea159efa38000836/train/elixir

defmodule FindOdd do
  def find(list) do
    list
      |> Enum.frequencies()
      |> Enum.filter(fn({_, v}) -> rem(v, 2) != 0 end)
      |> Enum.at(0)
      |> Kernel.elem(0)
  end
end

IO.inspect(FindOdd.find([20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]))
IO.inspect(FindOdd.find([7]))

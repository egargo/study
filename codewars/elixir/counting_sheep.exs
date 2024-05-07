# https://www.codewars.com/kata/54edbc7200b811e956000556/train/elixir

defmodule Shepherd do
  def count_sheeps(sheeps) do
    sheeps
      |> Enum.filter(fn(i) -> i == true end)
      |> Enum.count()
  end
end

IO.inspect(Shepherd.count_sheeps([
  true,  true,  true,  false,
  true,  true,  true,  true ,
  true,  false, true,  false,
  true,  false, false, true ,
  true,  true,  true,  true ,
  false, false, true,  true
]))

# https://www.codewars.com/kata/563e320cee5dddcf77000158/train/elixir

defmodule Calculator do
  def get_average(marks) do
    Kernel.div(Enum.sum(marks), Kernel.length(marks))
  end
end


IO.puts(Calculator.get_average([2,2,2,2]))
IO.puts(Calculator.get_average([1,2,3,4,5,]))
IO.puts(Calculator.get_average([1,1,1,1,1,1,1,2]))



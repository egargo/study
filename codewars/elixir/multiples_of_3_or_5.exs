# https://www.codewars.com/kata/514b92a657cdc65150000006/train/elixir

defmodule Challenge do
  def solution(number) when number > 0 do
    Enum.sum(Enum.filter(0..number - 1, fn(i) ->
      rem(i, 3) == 0 or rem(i, 5) == 0
    end))
  end

  def solution(number) when number <= 0 do
    0
  end
end

IO.inspect(Challenge.solution(2))
IO.inspect(Challenge.solution(12))
IO.inspect(Challenge.solution(22))
IO.inspect(Challenge.solution(0))
IO.inspect(Challenge.solution(-10))
IO.inspect(Challenge.solution(10))
IO.inspect(Challenge.solution(101))
IO.inspect(Challenge.solution(70))
IO.inspect(Challenge.solution(178))
IO.inspect(Challenge.solution(94))
IO.inspect(Challenge.solution(10))
IO.inspect(Challenge.solution(20))
IO.inspect(Challenge.solution(200))

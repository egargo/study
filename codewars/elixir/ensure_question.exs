# https://www.codewars.com/kata/5866fc43395d9138a7000006/train/elixir

defmodule Parsers do
  def ensure_question(s) do
    l = String.length(s)
    c = String.slice(s, l-1..l)
    case c  do
      "?" -> s
      _ -> s <> "?"
    end
  end
end

IO.puts(Parsers.ensure_question("Bee"))

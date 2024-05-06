# https://www.codewars.com/kata/56cd44e1aa4ac7879200010b/train/elixir

defmodule StringUtils do
  def upper_case?(str) do
    cond do
      String.upcase(str) == str -> true
      true -> false
    end
  end
end

IO.puts(StringUtils.upper_case?(""))
IO.puts(StringUtils.upper_case?("c"))
IO.puts(StringUtils.upper_case?("C"))
IO.puts(StringUtils.upper_case?("hello I AM DONALD"))
IO.puts(StringUtils.upper_case?("HELLO I AM DONALD"))
IO.puts(StringUtils.upper_case?("ACSKLDFJSgSKLDFJSKLDFJ"))
IO.puts(StringUtils.upper_case?("ACSKLDFJSGSKLDFJSKLDFJ"))


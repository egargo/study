# https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/elixir

defmodule Dna do
  def dna_strand(dna_str) do
    Enum.join(Enum.map(String.codepoints(dna_str), fn c ->
      case c do
        "A" -> "T"
        "T" -> "A"
        "G" -> "C"
        "C" -> "G"
      end
    end))
  end
end

IO.puts(Dna.dna_strand("ATTGC"))
IO.puts(Dna.dna_strand("GTAT"))

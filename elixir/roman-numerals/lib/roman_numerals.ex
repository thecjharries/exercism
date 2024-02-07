defmodule RomanNumerals do
  @doc """
  Convert the number to a roman number.
  """
  @spec numeral(pos_integer) :: String.t()
  def numeral(number) do

    roman_numerals = [
      {1000, "M"},
      {900, "CM"},
      {500, "D"},
      {400, "CD"},
      {100, "C"},
      {90, "XC"},
      {50, "L"},
      {40, "XL"},
      {10, "X"},
      {9, "IX"},
      {5, "V"},
      {4, "IV"},
      {1, "I"}
    ]

    div_rem = fn a, b -> {div(a, b), rem(a, b)} end

    roman_numerals
    |> Enum.reduce({number, ""}, fn {arabic, roman}, {n, acc} ->
      {div, rem} = div_rem.(n, arabic)
      {rem, acc <> String.duplicate(roman, div)}
    end)
    |> elem(1)
  end
end

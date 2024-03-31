defmodule Frequency do
  @doc """
  Count letter frequency in parallel.

  Returns a map of characters to frequencies.

  The number of worker processes to use can be set with 'workers'.
  """
  @spec frequency([String.t()], pos_integer) :: map
  def frequency(texts, workers) do
    texts
    |> Task.async_stream(fn text ->
        text
        |> String.downcase
        |> String.replace(~r/[^[:alpha:]]/u, "")
        |> String.codepoints
        |> Enum.frequencies
    end, workers: workers)
    |> Enum.reduce(%{}, fn {:ok, freq}, acc ->
      Map.merge(acc, freq, fn _key, acc_val, freq_val ->
        acc_val + freq_val
      end)
    end)
  end
end

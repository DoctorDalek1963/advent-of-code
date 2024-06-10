defmodule Util do
  @moduledoc """
    Provide some generic utility functions. Puzzle-specific functions should be elsewhere.
  """

  @doc """
    Read my real input from `input.txt`.
  """
  def get_input() do
    File.read!("./input.txt")
  end

  @doc """
    Treat the input as a list of integer separated by a delimiter, and return
    the list of parsed ints.
  """
  def parse_ints(input, delimiter \\ "\n") when is_bitstring(input) do
    input
    |> String.split(delimiter)
    |> Enum.map(fn x -> String.trim(x) end)
    |> Enum.filter(fn x -> x != "" end)
    |> Enum.map(fn x -> String.to_integer(x) end)
  end
end

defmodule Util do
  @moduledoc """
    Provide some generic utility functions. Puzzle-specific functions should be elsewhere.
  """

  @doc """
    Return the input given in the example, used for testing.
  """
  def get_test_input() do
    ""
  end

  @doc """
    Read my real input from `input.txt`.
  """
  def get_real_input() do
    File.read!("./input.txt")
  end

  @doc """
    Treat the input as a list of integer separated by a delimiter, and return
    the list of parsed ints.
  """
  def parse_ints(input, delimiter \\ "\n") when is_bitstring(input) do
    input
    |> String.split(delimiter)
    |> Enum.filter(fn x -> x != "" end)
    |> Enum.map(fn x -> String.to_integer(x) end)
  end
end

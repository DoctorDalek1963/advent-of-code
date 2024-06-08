defmodule Day1 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 1.
  """

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    Util.parse_ints(input)
    |> Enum.map(fn x -> calculate_fuel(x) end)
    |> Enum.sum()
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    Util.parse_ints(input)
    |> Enum.map(fn x -> recursively_calculate_fuel(x) end)
    |> Enum.sum()
  end

  @doc """
  Calculate the necessary fuel for this module based on its mass.

  Part 2 requires that negative values be clamped to 0, but this function will
  just return negative values if encountered.

  ## Examples
      iex> calculate_fuel(12)
      2
      iex> calculate_fuel(14)
      2
      iex> calculate_fuel(1969)
      654
      iex> calculate_fuel(100756)
      33583
      iex> calculate_fuel(2)
      -2
  """
  @spec calculate_fuel(integer()) :: integer()
  def calculate_fuel(mass) do
    div(mass, 3) - 2
  end

  @doc """
  Recursively calculate the necessary fuel for the given module mass.

  This function will respect the rules of part 2 and calculate the fuel
  required by fuel, clamping negative values to 0 in the process.

  ## Examples
      iex> recursively_calculate_fuel(12)
      2
      iex> recursively_calculate_fuel(14)
      2
      iex> recursively_calculate_fuel(1969)
      966
      iex> recursively_calculate_fuel(100756)
      50346
  """
  @spec recursively_calculate_fuel(integer()) :: integer()
  def recursively_calculate_fuel(mass) do
    case calculate_fuel(mass) do
      fuel when fuel > 0 -> fuel + recursively_calculate_fuel(fuel)
      fuel when fuel <= 0 -> 0
    end
  end
end

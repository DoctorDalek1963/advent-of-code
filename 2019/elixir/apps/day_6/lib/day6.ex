defmodule Day6 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 6.
  """

  @typedoc """
  A pair of celestial bodies, where the orbiter is in orbit around the orbitee.
  """
  @type orbit_pair() :: [orbiter: String.t(), orbitee: String.t()]

  @doc """
  Parse an orbit of the form "A)B".

  ## Examples
      iex> parse_orbit("A)B")
      [orbiter: "B", orbitee: "A"]
      iex> parse_orbit("ABC)DEF")
      [orbiter: "DEF", orbitee: "ABC"]
  """
  @spec parse_orbit(String.t()) :: orbit_pair()
  def parse_orbit(string) do
    [orbitee, orbiter] = String.split(string, ")")
    [orbiter: orbiter, orbitee: orbitee]
  end

  @doc """
  Count all the direct and indirect orbits that occur through the list of orbit pairs.

  Essentially, we're counting how many hops it takes for each item to reach "COM".

  ## Examples
      iex> count_orbits([
      ...>   [orbiter: "A", orbitee: "COM"],
      ...>   [orbiter: "B", orbitee: "A"],
      ...>   [orbiter: "C", orbitee: "A"]
      ...> ])
      %{
        "COM" => 0,
        "A" => 1,
        "B" => 2,
        "C" => 2
      }
  """
  @spec count_orbits([orbit_pair()]) :: %{String.t() => integer()}
  def count_orbits(orbit_list) do
    orbiter_list =
      orbit_list
      |> Enum.map(fn [orbiter: orbiter, orbitee: _] -> orbiter end)

    orbit_map =
      Map.new(
        orbit_list
        |> Enum.map(fn [orbiter: orbiter, orbitee: orbitee] -> {orbiter, orbitee} end)
      )

    count_map = %{"COM" => 0}

    count_orbits(orbiter_list, orbit_map, count_map)
  end

  # See `count_orbits/1`
  defp count_orbits(orbiter_list, orbit_map, count_map) do
    case orbiter_list do
      [orbiter | rest] ->
        count_orbits(rest, orbit_map, count_one_orbit(orbiter, orbit_map, count_map))

      [] ->
        count_map
    end
  end

  @doc """
  Count the number of hops from the given orbiter to reach "COM", moving only
  through the given orbit map.

  ## Examples
      iex> count_one_orbit("A", %{"A" => "COM"}, %{"COM" => 0})
      %{"COM" => 0, "A" => 1}
      iex> count_one_orbit("A", %{"B" => "A", "A" => "COM"}, %{"COM" => 0})
      %{"COM" => 0, "A" => 1}
      iex> count_one_orbit("B", %{"B" => "A", "A" => "COM"}, %{"COM" => 0})
      %{"COM" => 0, "A" => 1, "B" => 2}
  """
  @spec count_one_orbit(
          String.t(),
          %{String.t() => String.t()},
          %{String.t() => integer()}
        ) ::
          %{String.t() => integer()}
  def count_one_orbit(orbiter, orbit_map, count_map) do
    orbitee = Map.get(orbit_map, orbiter)

    case Map.get(count_map, orbitee) do
      count when is_integer(count) ->
        Map.put(count_map, orbiter, count + 1)

      nil ->
        new_count_map = count_one_orbit(orbitee, orbit_map, count_map)
        count_one_orbit(orbiter, orbit_map, new_count_map)
    end
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    input
    |> String.split("\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&parse_orbit/1)
    |> count_orbits()
    |> Map.values()
    |> Enum.sum()
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    0
  end
end

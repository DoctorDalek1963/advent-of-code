defmodule Day3 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 3.
  """

  @typedoc """
  A cardinal direction for movements.
  """
  @type direction() :: :left | :right | :up | :down

  @typedoc """
  A direction and a distance to move in that direction.
  """
  @type movement() :: {direction(), distance :: integer()}

  @typedoc """
  A coordinate on the 2D grid. `{x, y}` with positive `x` to the right and
  positive `y` going up.
  """
  @type coord() :: {x :: integer(), y :: integer()}

  @doc """
  Parse a string like `L3,U2,R1` into a list of movements.

  ## Example
      iex> parse_direction_list("L3,U2,R1")
      [left: 3, up: 2, right: 1]
      iex> parse_direction_list("D19,L12,U3,R4,D2")
      [down: 19, left: 12, up: 3, right: 4, down: 2]
  """
  @spec parse_direction_list(String.t()) :: [movement()]
  def parse_direction_list(input) do
    String.split(input, ",")
    |> Enum.map(fn instruction ->
      direction =
        case String.first(instruction) do
          "R" -> :right
          "L" -> :left
          "U" -> :up
          "D" -> :down
        end

      distance = String.to_integer(String.slice(instruction, 1, String.length(instruction)))

      {direction, distance}
    end)
  end

  @doc """
  Make a single movement from the starting point in the given direction for the
  given distance and return all the coordinates that the path will pass through
  (not including the starting point).

  ## Examples
      iex> move({0, 0}, {:right, 2})
      [{1, 0}, {2, 0}]
      iex> move({-2, 3}, {:up, 4})
      [{-2, 4}, {-2, 5}, {-2, 6}, {-2, 7}]
  """
  @spec move(coord(), movement()) :: [coord()]
  def move({sx, sy}, {direction, distance}) do
    1..distance
    |> Enum.map(fn d ->
      case direction do
        :right -> {sx + d, sy}
        :left -> {sx - d, sy}
        :up -> {sx, sy + d}
        :down -> {sx, sy - d}
      end
    end)
  end

  @doc """
  Return a list of every new point that the `movement_list` would take us
  through, if we started from the `starting_position`.

  ## Examples
      iex> make_movements({0, 0}, [left: 3, up: 2, right: 1])
      [{-1, 0}, {-2, 0}, {-3, 0}, {-3, 1}, {-3, 2}, {-2, 2}]
  """
  @spec make_movements(coord(), [movement()]) :: [coord()]
  def make_movements(starting_position, movement_list) do
    case movement_list do
      [movement | other_movements] ->
        new_coords = move(starting_position, movement)

        new_coords ++ make_movements(new_coords |> List.last(), other_movements)

      [] ->
        []
    end
  end

  @doc """
  Take two lines of input, parse them both with `parse_direction_list/1`, then
  get all the coordinates that those movements pass through (starting from
  `{0, 0}`) using `make_movements/2`.
  """
  @spec get_coords_lists(String.t()) :: {[coord()], [coord()]}
  def get_coords_lists(input) do
    [list1, list2 | _] = String.split(input, "\n")

    list1 = parse_direction_list(list1)
    list2 = parse_direction_list(list2)

    coords1 = make_movements({0, 0}, list1)
    coords2 = make_movements({0, 0}, list2)

    {coords1, coords2}
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    {coords1, coords2} = get_coords_lists(input)

    # We want the smallest Manhattan distance of the points where the paths intersect
    MapSet.intersection(MapSet.new(coords1), MapSet.new(coords2))
    |> Enum.map(fn {x, y} -> abs(x) + abs(y) end)
    |> Enum.min()
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    {coords1, coords2} = get_coords_lists(input)

    # We want the intersection where the sum of the steps that each path takes
    # to get there is minimised
    MapSet.intersection(MapSet.new(coords1), MapSet.new(coords2))
    |> Enum.map(fn c ->
      Enum.find_index(coords1, &(&1 == c)) + 1 + (Enum.find_index(coords2, &(&1 == c)) + 1)
    end)
    |> Enum.min()
  end
end

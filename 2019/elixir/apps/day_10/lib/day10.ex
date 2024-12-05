defmodule Day10 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 10.
  """

  @typedoc """
  An `{x, y}` position.
  """
  @type position() :: {integer(), integer()}

  @typedoc """
  A map of asteroid locations, indexed as `map[y][x]` with `{0, 0}` in the top left.
  """
  @type asteroid_map() :: %{position() => boolean()}

  @doc """
  Parse the string input into an `t:asteroid_map/0`.

  ## Examples
      iex> parse_asteroid_map("
      ...> .#..#
      ...> .....
      ...> #####
      ...> ....#
      ...> ...##
      ...> ")
      %{
        {0, 0} => false,
        {0, 1} => false,
        {0, 2} => true,
        {0, 3} => false,
        {0, 4} => false,
        {1, 0} => true,
        {1, 1} => false,
        {1, 2} => true,
        {1, 3} => false,
        {1, 4} => false,
        {2, 0} => false,
        {2, 1} => false,
        {2, 2} => true,
        {2, 3} => false,
        {2, 4} => false,
        {3, 0} => false,
        {3, 1} => false,
        {3, 2} => true,
        {3, 3} => false,
        {3, 4} => true,
        {4, 0} => true,
        {4, 1} => false,
        {4, 2} => true,
        {4, 3} => true,
        {4, 4} => true
      }
  """
  @spec parse_asteroid_map(String.t()) :: asteroid_map()
  def parse_asteroid_map(input) do
    String.split(input, "\n")
    |> Enum.filter(&(String.trim(&1) != ""))
    |> Enum.with_index()
    |> Enum.map(fn {line, y} ->
      String.to_charlist(line)
      |> Enum.filter(fn char -> char == ?# || char == ?. end)
      |> Enum.with_index()
      |> Enum.map(fn {char, x} -> {{x, y}, char == ?#} end)
    end)
    |> List.flatten()
    |> Map.new()
  end

  @doc """
  Find the best asteroid position to place a monitoring station, and return the
  position along with the count of all the other asteroids it can see.
  """
  @spec find_best_position_and_count(String.t() | asteroid_map()) :: {position(), integer()}
  def find_best_position_and_count(asteroid_map)

  def find_best_position_and_count(asteroid_map) when is_bitstring(asteroid_map) do
    parse_asteroid_map(asteroid_map)
    |> find_best_position_and_count()
  end

  def find_best_position_and_count(asteroid_map) when is_map(asteroid_map) do
    size = map_size(asteroid_map)

    max_position =
      0..size
      |> Enum.map(fn x ->
        0..size
        |> Enum.map(fn y -> {x, y} end)
      end)
      |> List.flatten()
      |> Enum.filter(fn {x, y} -> Map.has_key?(asteroid_map, {x, y}) end)
      |> List.last()

    {max_x, max_y} = max_position

    0..max_x
    |> Enum.map(fn x ->
      0..max_y
      |> Enum.map(fn y -> {{x, y}, count_one_position({x, y}, max_position, asteroid_map)} end)
    end)
    |> List.flatten()
    |> Enum.max_by(fn {_position, count} -> count end)
  end

  @doc """
  Count all the asteroids visible from this position.
  """
  @spec count_one_position(position(), position(), asteroid_map()) :: integer()
  def count_one_position(position, {max_x, max_y}, asteroid_map)
      when is_tuple(position) and is_map(asteroid_map) do
    0..max_x
    |> Enum.map(fn x ->
      0..max_y
      |> Enum.map(fn y -> visible_from?(position, {x, y}, asteroid_map) end)
    end)
    |> List.flatten()
    |> Enum.count(&(&1 == true))
  end

  @doc """
  Check if one asteroid is visible from another.
  """

  # ## Examples
  #     iex> map = parse_asteroid_map("
  #     ...> .#..#
  #     ...> .....
  #     ...> #####
  #     ...> ....#
  #     ...> ...##
  #     ...> ")
  #     iex> visible_from?({2, 2}, {4, 0}, map)
  #     true
  #     iex> visible_from?({4, 2}, {4, 3}, map)
  #     true
  #     iex> visible_from?({4, 2}, {4, 4}, map)
  #     false
  #     iex> visible_from?({1, 0}, {2, 2}, map)
  #     true
  #     iex> visible_from?({1, 0}, {3, 4}, map)
  #     false
  #     iex> visible_from?({1, 0}, {4, 4}, map)
  #     false
  #     iex> visible_from?({1, 0}, {1, 2}, map)
  #     true
  # """
  @spec visible_from?(position(), position(), asteroid_map()) :: boolean()
  def visible_from?(start_position, end_position, asteroid_map)

  def visible_from?(start_position, end_position, _asteroid_map)
      when start_position == end_position do
    false
  end

  def visible_from?(start_position, end_position, asteroid_map) when is_bitstring(asteroid_map) do
    visible_from?(start_position, end_position, parse_asteroid_map(asteroid_map))
  end

  def visible_from?({sx, sy}, {ex, ey}, asteroid_map) when is_map(asteroid_map) do
    case {Map.get(asteroid_map, {sx, sy}), Map.get(asteroid_map, {ex, ey})} do
      {true, true} ->
        dx = ex - sx
        dy = ey - sy

        {lim, sdx, sdy} =
          case {dx, dy} do
            {0, dy} ->
              {dy, 0, div(dy, abs(dy))}

            {dx, 0} ->
              {dx, div(dx, abs(dx)), 0}

            _ ->
              gcd = Integer.gcd(dx, dy)
              sdx = div(dx, gcd)
              sdy = div(dy, gcd)
              {gcd, sdx, sdy}
          end

        0..lim
        |> Enum.all?(fn mul ->
          x = sx + sdx * mul
          y = sy + sdy * mul
          {x, y} == {ex, ey} || Map.get(asteroid_map, {sx + sdx * mul, sy + sdy * mul}) == false
        end)

      _ ->
        false
    end
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    {_position, count} = find_best_position_and_count(input)
    count
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    0
  end
end

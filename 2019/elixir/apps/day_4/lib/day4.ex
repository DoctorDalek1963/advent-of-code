defmodule Day4 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 4.
  """

  @doc """
  Parse a range of integers from the input, where the two integers are separated by `-`.

  ## Examples
      iex> parse_range("324870-763981")
      {324870, 763981}
  """
  @spec parse_range(String.t()) :: {integer(), integer()}
  def parse_range(input) do
    [first, last] = String.split(input, "-") |> Enum.map(&String.trim/1)
    {String.to_integer(first), String.to_integer(last)}
  end

  @doc """
  Are all the digits in this password increasing?

  ## Examples
      iex> increasing?(123456)
      true
      iex> increasing?(123453)
      false
      iex> increasing?(123444)
      true
  """
  @spec increasing?(integer()) :: boolean()
  def increasing?(password)
      when is_integer(password) and password >= 100_000 and password <= 999_999 do
    {increasing, _} =
      Enum.reduce(to_charlist(to_string(password)), {true, 0}, fn num, acc ->
        case acc do
          {true, old_num} -> {num >= old_num, num}
          {false, _} -> {false, num}
        end
      end)

    increasing
  end

  @doc """
  Is this a possible password for part 1?

  A possible password must have all the digits in increasing order (see
  `increasing?/1`) and have at least one pair of identical digits next to each
  other. The pair of digits do not need to be isolated, unlike in part 2.

  ## Examples
      iex> possible_password_part1?(111111)
      true
      iex> possible_password_part1?(223450)
      false
      iex> possible_password_part1?(123789)
      false
      iex> possible_password_part1?(122779)
      true
      iex> possible_password_part1?(537291)
      false
      iex> possible_password_part1?(112341)
      false
      iex> possible_password_part1?(122222)
      true
      iex> possible_password_part1?(778999)
      true
      iex> possible_password_part1?(567888)
      true
      iex> possible_password_part1?(567889)
      true
  """
  @spec possible_password_part1?(integer()) :: boolean()
  def possible_password_part1?(password)
      when is_integer(password) and password >= 100_000 and password <= 999_999 do
    string_password = to_string(password)

    has_double_num =
      0..4
      |> Enum.map(&String.slice(string_password, &1, 2))
      |> Enum.any?(fn pair ->
        chars = to_charlist(pair)
        List.first(chars) == List.last(chars)
      end)

    has_double_num && increasing?(password)
  end

  @doc """
  Is this a possible password for part 2?

  A possible password for part 2 has the same requirements as part 1 (see
  `possible_password_part1?/1`), except that we now require the pair of
  identical digits to be isolated from other digits of that type.

  ## Examples
      iex> possible_password_part2?(111111)
      false
      iex> possible_password_part2?(223450)
      false
      iex> possible_password_part2?(123789)
      false
      iex> possible_password_part2?(122779)
      true
      iex> possible_password_part2?(537291)
      false
      iex> possible_password_part2?(112341)
      false
      iex> possible_password_part2?(122222)
      false
      iex> possible_password_part2?(778999)
      true
      iex> possible_password_part2?(567888)
      false
      iex> possible_password_part2?(567889)
      true
  """
  @spec possible_password_part2?(integer()) :: boolean()
  def possible_password_part2?(password)
      when is_integer(password) and password >= 100_000 and password <= 999_999 do
    has_isolated_double_num =
      count_consecutive(to_charlist(to_string(password)))
      |> Enum.any?(fn {_num, count} -> count == 2 end)

    has_isolated_double_num && increasing?(password)
  end

  @doc """
  Count consecutive items in a list and return a list of their counts.

  ## Examples
      iex> count_consecutive([:a, :a, :b, :c, :c, :c])
      [a: 2, b: 1, c: 3]
      iex> count_consecutive([1, 2, 2, 2, 3, 7, 1, 1, 3, 3])
      [{1, 1}, {2, 3}, {3, 1}, {7, 1}, {1, 2}, {3, 2}]
  """
  @spec count_consecutive([term], [{term, integer()}]) :: [{term, integer()}]
  def count_consecutive(items, counts \\ []) do
    case items do
      [item | rest] ->
        new_counts =
          case List.last(counts) do
            {x, count} when x === item -> List.replace_at(counts, -1, {x, count + 1})
            _ -> counts ++ [{item, 1}]
          end

        count_consecutive(rest, new_counts)

      [] ->
        counts
    end
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    {first, last} = parse_range(input)
    first..last |> Enum.count(&possible_password_part1?/1)
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    {first, last} = parse_range(input)
    first..last |> Enum.count(&possible_password_part2?/1)
  end
end

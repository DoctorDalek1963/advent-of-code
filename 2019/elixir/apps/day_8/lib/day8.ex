defmodule Day8 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 8.
  """

  @typedoc """
  A layer of the image. Consists of a list of rows, each of which is a charlist
  where each character represents a pixel.
  """
  @type layer() :: [charlist()]

  @doc """
  Parse a single layer of size `width * height` from the charlist.

  This function does not care about any extra characters at the end of list,
  nor does it return a new list which removes the parsed characters.

  ## Examples
      iex> parse_layer(~c"1234567890", 3, 2)
      [~c"123", ~c"456"]
  """
  @spec parse_layer(charlist(), integer(), integer()) :: layer()
  def parse_layer(pixels, width, height)
      when is_list(pixels) and is_integer(width) and is_integer(height) do
    1..height
    |> Enum.map(fn row_num ->
      Enum.take(Enum.drop(pixels, width * (row_num - 1)), width)
    end)
  end

  @doc """
  Parse all the layers from the string or charlist.

  ## Examples
      iex> parse_all_layers("123456789012", 3, 2)
      [[~c"123", ~c"456"], [~c"789", ~c"012"]]
  """
  @spec parse_all_layers(String.t() | charlist(), integer(), integer()) :: [layer()]
  def parse_all_layers(input, width, height)
      when is_integer(width) and is_integer(height) do
    chars = to_charlist(input)
    layer_count = div(length(chars), width * height)

    1..layer_count
    |> Enum.map(fn layer_num ->
      parse_layer(Enum.drop(chars, width * height * (layer_num - 1)), width, height)
    end)
  end

  @doc """
  Count how many pixels in the layer match the given character.

  ## Examples
      iex> count_matching_in_layer([~c"0012", ~c"1021", ~c"0120"], ?0)
      5
      iex> count_matching_in_layer([~c"0012", ~c"1021", ~c"0120"], ?1)
      4
      iex> count_matching_in_layer([~c"0012", ~c"1021", ~c"0120"], ?2)
      3
  """
  @spec count_matching_in_layer(layer(), char()) :: integer()
  def count_matching_in_layer(layer, desired_char) do
    layer
    |> Enum.map(fn row ->
      row |> Enum.count(&(&1 === desired_char))
    end)
    |> Enum.sum()
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    layers = parse_all_layers(input, 25, 6)

    %{zeroes: _, ones: ones, twos: twos} =
      Enum.map(layers, fn layer ->
        %{
          zeroes: count_matching_in_layer(layer, ?0),
          ones: count_matching_in_layer(layer, ?1),
          twos: count_matching_in_layer(layer, ?2)
        }
      end)
      |> Enum.min_by(fn map -> map.zeroes end)

    ones * twos
  end

  @doc """
  Calculate the pixel at this position by combining all the layers. A `?0` or
  `?1` stops the recursion, but a `?2` is a transparent pixel, so we will
  recurse to the next layer.
  """
  @spec calculate_pixel([layer()], integer(), integer()) :: char()
  def calculate_pixel([top_layer | other_layers], x, y) do
    case Enum.at(top_layer, y) |> Enum.at(x) do
      ?0 -> ?0
      ?1 -> ?1
      ?2 -> calculate_pixel(other_layers, x, y)
    end
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: layer()
  def process_part2(input) do
    layers = parse_all_layers(input, 25, 6)

    0..5
    |> Enum.map(fn y ->
      0..24
      |> Enum.map(fn x -> calculate_pixel(layers, x, y) end)
    end)
  end
end

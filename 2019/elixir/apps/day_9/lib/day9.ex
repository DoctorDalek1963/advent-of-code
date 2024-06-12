defmodule Day9 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 9.
  """

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    int_pid = Intcode.Util.start_interpreter(input, 0, true)

    receive do
      :awaiting_input -> send(int_pid, {:input, 1})
    end

    [value] = Intcode.Util.consume_outputs()

    value
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    int_pid = Intcode.Util.start_interpreter(input, 0, true)

    receive do
      :awaiting_input -> send(int_pid, {:input, 2})
    end

    [value] = Intcode.Util.consume_outputs()

    value
  end
end

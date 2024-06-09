defmodule Day5 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 5.
  """

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    int_pid = Intcode.Util.start_interpreter(input)

    receive do
      :awaiting_input -> send(int_pid, {:input, 1})
    end

    [answer | zeroes] = Intcode.Util.consume_outputs()

    if Enum.all?(zeroes, &(&1 === 0)) do
      answer
    else
      {:error, "expected all zeroes for diagnostic outputs", zeroes}
    end
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    int_pid = Intcode.Util.start_interpreter(input)

    receive do
      :awaiting_input -> send(int_pid, {:input, 5})
    end

    [answer | zeroes] = Intcode.Util.consume_outputs()

    if Enum.all?(zeroes, &(&1 === 0)) do
      answer
    else
      {:error, "expected all zeroes for diagnostic outputs", zeroes}
    end
  end
end

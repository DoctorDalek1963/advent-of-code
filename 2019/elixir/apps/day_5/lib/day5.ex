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

    [answer | zeroes] = consume_outputs()

    if Enum.all?(zeroes, &(&1 === 0)) do
      answer
    else
      {:error, "expected all zeroes for diagnostic outputs", zeroes}
    end
  end

  @doc """
  Receive `{:output, value}` messages from the interpreter and create a list,
  placing the most recently received value at the front. This function will
  consume every `{:output, value}` message until a different message is
  received.
  """
  @spec consume_outputs([integer()]) :: [integer()]
  def consume_outputs(list \\ []) do
    receive do
      {:output, value} when is_integer(value) -> consume_outputs([value | list])
      _ -> list
    end
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    0
  end
end

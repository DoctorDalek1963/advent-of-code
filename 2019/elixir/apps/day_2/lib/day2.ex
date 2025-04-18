defmodule Day2 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 2.
  """

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    {:halted, memory} =
      Util.parse_ints(input, ",")
      |> List.replace_at(1, 12)
      |> List.replace_at(2, 2)
      |> Intcode.Util.interpret_no_io()

    Enum.at(memory, 0)
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    bytecode = Util.parse_ints(input, ",")

    0..100
    |> Enum.each(fn noun ->
      spawn(Day2, :part2_helper, [self(), bytecode, noun, 19_690_720])
    end)

    receive do
      [noun: noun, verb: verb] ->
        100 * noun + verb
    end
  end

  @doc """
  A helper function for `process_part2/1`.

  Given the starting bytecode and a noun, this function will try every verb in
  `0..100` and if the interpreter produces the desired value, then it will send
  `[noun: noun, verb: verb]` back to `parent_pid`.
  """
  @spec part2_helper(pid(), [integer()], integer(), integer()) :: nil
  def part2_helper(parent_pid, bytecode, noun, desired_value) do
    results =
      0..100
      |> Enum.map(fn verb ->
        {verb,
         bytecode
         |> List.replace_at(1, noun)
         |> List.replace_at(2, verb)
         |> Intcode.Util.interpret_no_io()}
      end)
      |> Enum.filter(fn x ->
        case x do
          {_, {:halted, [value | _]}} when value == desired_value -> true
          _ -> false
        end
      end)
      |> Enum.map(fn x ->
        case x do
          {verb, {:halted, [value | _]}} when value == desired_value -> verb
        end
      end)

    case results do
      [verb] ->
        send(parent_pid, noun: noun, verb: verb)
        nil

      [] ->
        nil
    end
  end
end

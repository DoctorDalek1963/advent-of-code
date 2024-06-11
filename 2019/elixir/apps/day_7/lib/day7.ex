defmodule Day7 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 7.
  """

  @doc """
  Run a single amplifier, providing the phase setting and input value as
  inputs, and returning the amplifier's output.
  """
  def run_one_amplifier(phase_setting, program, input) do
    int_pid = Intcode.Util.start_interpreter(program)

    receive do
      :awaiting_input -> send(int_pid, {:input, phase_setting})
    end

    receive do
      :awaiting_input -> send(int_pid, {:input, input})
    end

    value =
      receive do
        {:output, value} -> value
      end

    receive do
      {:halted, _} -> nil
    end

    value
  end

  @doc """
  Run a sequence of amplifiers, feeding the output of one into the input of the next.

  Note that we pop phase settings from the front of the list, so the first
  amplifier will get the phase setting at the end of the list, and will be
  provided with the value 0.
  """
  @spec run_amplifiers([integer()], String.t() | [integer()]) :: nil
  def run_amplifiers(phase_settings, program)

  # This is the last phase setting, so it must be amplifier 1, which receives an initial input of 0
  def run_amplifiers([phase_setting], program) do
    run_one_amplifier(phase_setting, program, 0)
  end

  def run_amplifiers([phase_setting | rest], program) do
    run_one_amplifier(phase_setting, program, run_amplifiers(rest, program))
  end

  @doc """
  Permute the given list to find all permutations.

  ## Examples
      iex> permute([1, 2, 3])
      [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
  """
  @spec permute([value]) :: [[value]] when value: var
  def permute(list) when is_list(list) do
    if list === [] do
      [[]]
    else
      for head <- list, tail <- permute(list -- [head]) do
        [head | tail]
      end
    end
  end

  @doc """
  Process the first part of the puzzle.
  """
  @spec process_part1(String.t()) :: integer()
  def process_part1(input) do
    {:ok, number} =
      permute([0, 1, 2, 3, 4])
      |> Task.async_stream(fn phase_settings -> run_amplifiers(phase_settings, input) end)
      |> Enum.max()

    number
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    0
  end
end

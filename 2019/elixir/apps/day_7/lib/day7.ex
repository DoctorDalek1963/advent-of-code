defmodule Day7 do
  @moduledoc """
  Solve the Advent of Code puzzles for 2019 day 7.
  """

  @doc """
  Start the interpreter for this amplifier. If this is amplifier number 1, we
  will seed it with the input 0. This function will start the interpreter and
  immediately defer to `run_amplifier_inner_loop/3`.
  """
  @spec run_amplifier(pid(), integer(), String.t() | [integer()], integer()) :: nil
  def run_amplifier(parent_pid, amp_num, program, phase_setting) do
    int_pid = Intcode.Util.start_interpreter(program)

    receive do
      :awaiting_input -> send(int_pid, {:input, phase_setting})
    end

    if amp_num === 1 do
      receive do
        :awaiting_input -> send(int_pid, {:input, 0})
      end
    end

    run_amplifier_inner_loop(parent_pid, int_pid, amp_num)
  end

  @doc """
  This loop facilitates 2-way communication between the parent and the
  interpreter for this amplifier.

  If we receive a signal from the interpreter (see the `Intcode.Interpreter`
  docs for information on messages), then we forward the output to the parent
  as `%{amp: amp_num, output: value}` or otherwise ignore it. If the parent
  sends a `{:input, value}`, then we forward it to the interpreter if it's
  alive, otherwise we send `%{amp: amp_num, msg: :interpreter_dead}` back to
  the parent.
  """
  @spec run_amplifier_inner_loop(pid(), pid(), integer()) :: nil
  def run_amplifier_inner_loop(parent_pid, int_pid, amp_num) do
    receive do
      :awaiting_input ->
        nil

      {:output, value} ->
        send(parent_pid, %{amp: amp_num, output: value})

      {:input, value} ->
        if Process.alive?(int_pid) do
          send(int_pid, {:input, value})
        else
          send(parent_pid, %{amp: amp_num, msg: :interpreter_dead})
        end

      {:halted, _} ->
        nil
    end

    run_amplifier_inner_loop(parent_pid, int_pid, amp_num)
  end

  @doc """
  Run the Amplifier Controller Software across 5 amplifiers.

  The `phase_settings` list is in order, so amplifier 1 gets the first element
  and amplifier 5 gets the last one.
  """
  @spec run_amplifier_controller_software(String.t() | [integer()], [integer()]) :: integer()
  def run_amplifier_controller_software(program, phase_settings)
      when is_list(phase_settings) and length(phase_settings) === 5 do
    amp_pids =
      phase_settings
      |> Enum.zip([1, 2, 3, 4, 5])
      |> Enum.map(fn {phase_setting, amp_num} ->
        parent = self()
        spawn(fn -> run_amplifier(parent, amp_num, program, phase_setting) end)
      end)

    controller_software_inner_loop(amp_pids)
  end

  @doc """
  Control and coordinate the 5 amplifiers. This function passes outputs into
  the next amplifier and stops once the interpreter for amplifier 1 has died.
  It then returns the most recent value from amplifier 5.
  """
  @spec controller_software_inner_loop([pid()], integer()) :: integer()
  def controller_software_inner_loop(amp_pids, most_recent_value \\ -999) do
    receive do
      %{amp: amp_num, output: value} ->
        next_amp_pid = Enum.at(amp_pids, rem(amp_num, 5))
        send(next_amp_pid, {:input, value})
        controller_software_inner_loop(amp_pids, value)

      %{amp: 1, msg: :interpreter_dead} ->
        most_recent_value
    end
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
      |> Task.async_stream(fn phase_settings ->
        run_amplifier_controller_software(input, phase_settings)
      end)
      |> Enum.max()

    number
  end

  @doc """
  Process the second part of the puzzle.
  """
  @spec process_part2(String.t()) :: integer()
  def process_part2(input) do
    {:ok, number} =
      permute([5, 6, 7, 8, 9])
      |> Task.async_stream(fn phase_settings ->
        run_amplifier_controller_software(input, phase_settings)
      end)
      |> Enum.max()

    number
  end
end

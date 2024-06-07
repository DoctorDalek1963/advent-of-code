defmodule IntCode.Interpreter do
  @moduledoc """
  Interpret the `IntCode` language built for AOC 2019.

  `IntCode` is a system of bytecode, which operates on a list of integers. This
  interpreter is implemented in a way that makes use of message passing to
  interface with the user.

  ## Messages

  When the interpreter encounters an input instruction, it will send
  `:awaiting_input` to the user. It will then wait for a response, which should
  be of the form `{:input, value}`, where `value` is an integer.

  When it encounters an output instruction, it will send `{:output, value}` to
  the user, where `value` is an integer.

  When it encounters a halt instruction, it will send `{:halted, memory}`,
  where `memory` is a snapshot of the memory when the interpreter halted.

  If the interpreter runs into an error, it will send `{:error, message}`,
  where `message` is a string.

  ## Opcodes

  `[1, $r1, $r2, $r3]` => Fetch the values at `$r1` and `$r2`, add them, and
  store the result at the address in `$r3`.

  `[2, $r1, $r2, $r3]` => Fetch the values at `$r1` and `$r2`, multiply them,
  and store the result at the address in `$r3`.

  `[99]` => Halt immediately, returning the current state of the interpreter's memory.

  """

  @typedoc """
  A snapshot of the interpreter's memory.
  """
  @type memory() :: [integer()]

  @doc """
  Interpret the given bytecode.

  The first argument is the PID of the user. The interpreter will send messages
  to the user for I/O operations. See the module docs for more details.
  """
  @spec interpret(pid(), memory(), integer()) :: nil
  def interpret(user_pid, bytecode, program_counter \\ 0)
      when is_pid(user_pid) and is_list(bytecode) and is_integer(program_counter) do
    case Enum.slice(bytecode, program_counter, length(bytecode)) do
      # Add
      [1, r1, r2, r3 | _] ->
        interpret(
          user_pid,
          List.replace_at(bytecode, r3, Enum.at(bytecode, r1) + Enum.at(bytecode, r2)),
          program_counter + 4
        )

      # Multiply
      [2, r1, r2, r3 | _] ->
        interpret(
          user_pid,
          List.replace_at(bytecode, r3, Enum.at(bytecode, r1) * Enum.at(bytecode, r2)),
          program_counter + 4
        )

      # Halt
      [99 | _] ->
        send(user_pid, {:halted, bytecode})

      [] ->
        send(user_pid, {:error, "program counter out of range"})
    end
  end
end

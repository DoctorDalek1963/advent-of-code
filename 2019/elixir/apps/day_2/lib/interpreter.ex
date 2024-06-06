defmodule Interpreter do
  @moduledoc """
  Interpret the `IntCode` language built for AOC 2019.
  """

  @doc """
  Interpret the given bytecode, starting from the specified program counter, or from 0 if not specified.

  This function will recurse until it reaches the opcode 99, which will cause it to halt.

  ## Opcodes

  `[1, $r1, $r2, $r3]` => Fetch the values at `$r1` and `$r2`, add them, and store the result at the address in `$r3`.

  `[2, $r1, $r2, $r3]` => Fetch the values at `$r1` and `$r2`, multiply them, and store the result at the address in `$r3`.

  `[99]` => Halt immediately, returning the current state of the interpreter's memory.

  ## Examples
      iex> Interpreter.interpret([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])
      {:ok, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]}
      iex> Interpreter.interpret([1, 0, 0, 0, 99])
      {:ok, [2, 0, 0, 0, 99]}
      iex> Interpreter.interpret([2, 3, 0, 3, 99])
      {:ok, [2, 3, 0, 6, 99]}
      iex> Interpreter.interpret([2, 4, 4, 5, 99, 0])
      {:ok, [2, 4, 4, 5, 99, 9801]}
      iex> Interpreter.interpret([1, 1, 1, 4, 99, 5, 6, 0, 99])
      {:ok, [30, 1, 1, 4, 2, 5, 6, 0, 99]}
  """
  def interpret(bytecode, program_counter \\ 0) when is_list(bytecode) do
    case Enum.slice(bytecode, program_counter, length(bytecode)) do
      # Add
      [1, r1, r2, r3 | _] ->
        interpret(
          List.replace_at(bytecode, r3, Enum.at(bytecode, r1) + Enum.at(bytecode, r2)),
          program_counter + 4
        )

      # Multiply
      [2, r1, r2, r3 | _] ->
        interpret(
          List.replace_at(bytecode, r3, Enum.at(bytecode, r1) * Enum.at(bytecode, r2)),
          program_counter + 4
        )

      # Halt
      [99 | _] ->
        {:ok, bytecode}

      [] ->
        {:error, "program counter out of range"}
    end
  end
end

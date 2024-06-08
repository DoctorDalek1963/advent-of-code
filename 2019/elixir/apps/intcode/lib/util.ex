defmodule Intcode.Util do
  @moduledoc """
  Some simple utility functions for starting and interacting with the `Intcode.Interpreter`.
  """

  @doc """
  Start the interpreter and return its PID to allow communication. See the docs
  of `Intcode.Interpreter` for message formats.

  If given a string, parse it as a comma-separated list of integers and start
  the interpreter with that bytecode. If given a list of integers, then just
  use that as the bytecode.
  """
  def start_interpreter(bytecode, program_counter \\ 0)

  @spec start_interpreter(String.t(), integer()) :: pid()
  def start_interpreter(input, program_counter) when is_bitstring(input) do
    String.split(input, ",")
    |> Enum.map(&(&1 |> String.trim() |> String.to_integer()))
    |> start_interpreter(program_counter)
  end

  @spec start_interpreter([integer()], integer()) :: pid()
  def start_interpreter(bytecode, program_counter) when is_list(bytecode) do
    spawn(Intcode.Interpreter, :interpret, [self(), bytecode, program_counter])
  end

  @doc """
  Interpret the given bytecode, assuming that no I/O will be necessary.

  This function will return `{:halted, memory}` if the interpreter was
  successful, `{:error, message}` if it failed internally, or `{:unexpected_io,
  msg}` if it received an unexpected I/O message from the interpreter.
  """
  @spec interpret_no_io(String.t() | [integer()], integer()) ::
          {:halted, [integer()]} | {:error, String.t()} | {:unexpected_io, any()}
  def interpret_no_io(bytecode, program_counter \\ 0) do
    start_interpreter(bytecode, program_counter)

    receive do
      {:halted, memory} -> {:halted, memory}
      {:error, message} -> {:error, message}
      msg -> {:unexpected_io, msg}
    end
  end
end

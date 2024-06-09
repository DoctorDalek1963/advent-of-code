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

  @doc """
  Receive `{:output, value}` messages from the interpreter and create a list,
  placing the most recently received value at the front.

  This function will consume every `{:output, value}` message until a different
  message is received.

  Note that if a different message its received, it will re-send that message
  to `self/0`. This will place it back in this process' mailbox, but it's not
  guaranteed to preserve the order if another message came in while we were
  re-sending that one.
  """
  @spec consume_outputs([integer()]) :: [integer()]
  def consume_outputs(list \\ []) do
    receive do
      {:output, value} when is_integer(value) ->
        consume_outputs([value | list])

      other_msg ->
        send(self(), other_msg)
        list
    end
  end
end

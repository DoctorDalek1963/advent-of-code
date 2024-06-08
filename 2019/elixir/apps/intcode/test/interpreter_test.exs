defmodule IntcodeInterpreterTest do
  use ExUnit.Case
  doctest Intcode.Interpreter, import: true
  doctest Intcode.Util, import: true

  import Intcode.Util

  @doc """
  Assert that the message is received and send the given reply.

  Equivalent to:

      receive do
        $message -> $action
      after
        $timeout -> assert false, $error_string
      end
  """
  defmacro assert_reply(
             message,
             action,
             timeout \\ 100,
             error_string \\ "interpreter failed to send expected message within a second"
           ) do
    quote do
      receive do
        unquote(message) -> unquote(action)
      after
        unquote(timeout) -> assert false, unquote(error_string)
      end
    end
  end

  test "halt" do
    assert interpret_no_io([99]) === {:halted, [99]}
    assert interpret_no_io([99, 1, 3, 65, 2, 7, 223]) === {:halted, [99, 1, 3, 65, 2, 7, 223]}
  end

  test "add" do
    assert interpret_no_io([1, 0, 0, 0, 99]) === {:halted, [2, 0, 0, 0, 99]}
    assert interpret_no_io([1101, 12, -1, 0, 99]) === {:halted, [11, 12, -1, 0, 99]}
    assert interpret_no_io([101, 12, 1, 1, 99]) === {:halted, [101, 24, 1, 1, 99]}
  end

  test "multiply" do
    assert interpret_no_io([2, 4, 4, 5, 99, 0]) === {:halted, [2, 4, 4, 5, 99, 9801]}
    assert interpret_no_io([1102, 4, 4, 5, 99, 0]) === {:halted, [1102, 4, 4, 5, 99, 16]}
    assert interpret_no_io([1002, 2, 10, 0, 99]) === {:halted, [100, 2, 10, 0, 99]}
  end

  test "input" do
    int_pid = start_interpreter([3, 0, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 11}))
    assert_receive {:halted, [11, 0, 99]}
    Process.exit(int_pid, :kill)

    int_pid = start_interpreter([3, 1, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, -102}))
    assert_receive {:halted, [3, -102, 99]}
    Process.exit(int_pid, :kill)
  end

  test "output" do
    int_pid = start_interpreter([4, 2, 99])
    assert_receive {:output, 99}
    assert_receive {:halted, [4, 2, 99]}
    Process.exit(int_pid, :kill)

    int_pid = start_interpreter([104, 506, 99])
    assert_receive {:output, 506}
    assert_receive {:halted, [104, 506, 99]}
    Process.exit(int_pid, :kill)
  end

  test "combinations" do
    # Add and multiply
    assert interpret_no_io([1, 1, 1, 4, 99, 5, 6, 0, 99]) ===
             {:halted, [30, 1, 1, 4, 2, 5, 6, 0, 99]}

    # Add with input
    int_pid = start_interpreter([3, 1, 101, 10, 1, 0, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 42}))
    assert_receive {:halted, [52, 42, 101, 10, 1, 0, 99]}
    Process.exit(int_pid, :kill)

    # Multiply with input
    int_pid = start_interpreter([3, 1, 102, 10, 1, 0, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 42}))
    assert_receive {:halted, [420, 42, 102, 10, 1, 0, 99]}
    Process.exit(int_pid, :kill)
  end

  test "errors" do
    assert interpret_no_io([]) === {:error, "program counter out of bounds (address 0)"}

    assert interpret_no_io([1, 0, 0, 0, 99], 10) ===
             {:error, "program counter out of bounds (address 10)"}

    assert interpret_no_io([0]) === {:error, "unrecognised opcode 0 at address 0"}

    assert interpret_no_io([1101, 3, 4, 0, 98]) ===
             {:error, "unrecognised opcode 98 at address 4"}
  end
end

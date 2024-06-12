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

    int_pid = start_interpreter([3, 1, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, -102}))
    assert_receive {:halted, [3, -102, 99]}
  end

  test "output" do
    start_interpreter([4, 2, 99])
    assert_receive {:output, 99}
    assert_receive {:halted, [4, 2, 99]}

    start_interpreter([104, 506, 99])
    assert_receive {:output, 506}
    assert_receive {:halted, [104, 506, 99]}
  end

  test "equality" do
    # Equality with I/O in position mode
    int_pid = start_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, 8}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]}

    int_pid = start_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, -2}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]}

    int_pid = start_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, 230}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]}

    # Equality with I/O in immediate mode
    int_pid = start_interpreter([3, 3, 1108, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 8}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 3, 1108, 1, 8, 3, 4, 3, 99]}

    int_pid = start_interpreter([3, 3, 1108, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 3}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 3, 1108, 0, 8, 3, 4, 3, 99]}

    int_pid = start_interpreter([3, 3, 1108, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 3562}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 3, 1108, 0, 8, 3, 4, 3, 99]}
  end

  test "less than" do
    # Less than with I/O in position mode
    int_pid = start_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, 2}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]}

    int_pid = start_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, -100}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]}

    int_pid = start_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, 100}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]}

    int_pid = start_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8])
    assert_reply(:awaiting_input, send(int_pid, {:input, 8}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]}

    # Less than with I/O in immediate mode
    int_pid = start_interpreter([3, 3, 1107, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 2}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 3, 1107, 1, 8, 3, 4, 3, 99]}

    int_pid = start_interpreter([3, 3, 1107, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, -100}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 3, 1107, 1, 8, 3, 4, 3, 99]}

    int_pid = start_interpreter([3, 3, 1107, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 100}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 3, 1107, 0, 8, 3, 4, 3, 99]}

    int_pid = start_interpreter([3, 3, 1107, -1, 8, 3, 4, 3, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 8}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 3, 1107, 0, 8, 3, 4, 3, 99]}
  end

  test "jump with I/O equality check" do
    # Position mode
    int_pid = start_interpreter([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9])
    assert_reply(:awaiting_input, send(int_pid, {:input, 0}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9]}

    int_pid = start_interpreter([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9])
    assert_reply(:awaiting_input, send(int_pid, {:input, 1}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 1, 1, 1, 9]}

    int_pid = start_interpreter([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9])
    assert_reply(:awaiting_input, send(int_pid, {:input, 13_906}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 13_906, 1, 1, 9]}

    # Immediate mode
    int_pid = start_interpreter([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
    assert_reply(:awaiting_input, send(int_pid, {:input, 0}))
    assert_receive {:output, 0}
    assert_receive {:halted, [3, 3, 1105, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 0]}

    int_pid = start_interpreter([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
    assert_reply(:awaiting_input, send(int_pid, {:input, 1}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]}

    int_pid = start_interpreter([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
    assert_reply(:awaiting_input, send(int_pid, {:input, 13_906}))
    assert_receive {:output, 1}
    assert_receive {:halted, [3, 3, 1105, 13_906, 9, 1101, 0, 0, 12, 4, 12, 99, 1]}
  end

  test "relative mode" do
    quine = [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    start_interpreter(quine, 0, true)
    quine_output = consume_outputs() |> Enum.reverse()
    assert quine === quine_output
  end

  test "large numbers" do
    start_interpreter([1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0])
    assert_receive {:output, 1_219_070_632_396_864}
    assert_receive {:halted, [1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 1_219_070_632_396_864]}

    start_interpreter([104, 1_125_899_906_842_624, 99])
    assert_receive {:output, 1_125_899_906_842_624}
    assert_receive {:halted, [104, 1_125_899_906_842_624, 99]}
  end

  test "combinations" do
    # Add and multiply
    assert interpret_no_io([1, 1, 1, 4, 99, 5, 6, 0, 99]) ===
             {:halted, [30, 1, 1, 4, 2, 5, 6, 0, 99]}

    # Add with input
    int_pid = start_interpreter([3, 1, 101, 10, 1, 0, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 42}))
    assert_receive {:halted, [52, 42, 101, 10, 1, 0, 99]}

    # Multiply with input
    int_pid = start_interpreter([3, 1, 102, 10, 1, 0, 99])
    assert_reply(:awaiting_input, send(int_pid, {:input, 42}))
    assert_receive {:halted, [420, 42, 102, 10, 1, 0, 99]}

    # Check for 8 (taken from the end of day 5 part 2)
    int_pid =
      start_interpreter(
        [3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20] ++
          [1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125] ++
          [20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101] ++
          [1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
      )

    assert_reply(:awaiting_input, send(int_pid, {:input, -2}))
    assert_receive {:output, 999}
    assert_receive {:halted, _}

    int_pid =
      start_interpreter(
        [3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20] ++
          [1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125] ++
          [20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101] ++
          [1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
      )

    assert_reply(:awaiting_input, send(int_pid, {:input, 8}))
    assert_receive {:output, 1000}
    assert_receive {:halted, _}

    int_pid =
      start_interpreter(
        [3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20] ++
          [1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125] ++
          [20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101] ++
          [1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
      )

    assert_reply(:awaiting_input, send(int_pid, {:input, 100}))
    assert_receive {:output, 1001}
    assert_receive {:halted, _}
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

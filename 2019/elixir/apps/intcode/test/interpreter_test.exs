defmodule IntCodeInterpreterTest do
  use ExUnit.Case
  doctest IntCode.Interpreter, import: true
  doctest IntCode.Util, import: true

  import IntCode.Util

  test "add, multiply, and halt" do
    assert interpret_no_io([99]) === {:halted, [99]}
    assert interpret_no_io([2, 4, 4, 5, 99, 0]) === {:halted, [2, 4, 4, 5, 99, 9801]}

    assert interpret_no_io([1, 1, 1, 4, 99, 5, 6, 0, 99]) ===
             {:halted, [30, 1, 1, 4, 2, 5, 6, 0, 99]}
  end

  test "errors" do
    assert interpret_no_io([]) === {:error, "program counter out of bounds"}
    assert interpret_no_io([1, 0, 0, 0, 99], 10) === {:error, "program counter out of bounds"}
  end
end

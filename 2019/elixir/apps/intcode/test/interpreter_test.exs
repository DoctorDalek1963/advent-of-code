defmodule IntCodeInterpreterTest do
  use ExUnit.Case
  doctest IntCode.Interpreter

  import IntCode.Interpreter

  test "add, multiply, and halt" do
    assert interpret([99]) === {:ok, [99]}
    assert interpret([2, 4, 4, 5, 99, 0]) === {:ok, [2, 4, 4, 5, 99, 9801]}
    assert interpret([1, 1, 1, 4, 99, 5, 6, 0, 99]) === {:ok, [30, 1, 1, 4, 2, 5, 6, 0, 99]}
  end

  test "errors" do
    assert interpret([]) === {:error, "program counter out of range"}
    assert interpret([1, 0, 0, 0, 99], 10) === {:error, "program counter out of range"}
  end
end

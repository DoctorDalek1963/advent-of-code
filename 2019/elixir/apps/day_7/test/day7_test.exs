defmodule Day7Test do
  use ExUnit.Case
  doctest Day7, import: true

  test "thruster signals" do
    assert Day7.run_amplifiers(
             [0, 1, 2, 3, 4],
             [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
           ) ===
             43_210

    assert Day7.run_amplifiers(
             [4, 3, 2, 1, 0],
             [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23] ++
               [101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]
           ) ===
             54_321

    assert Day7.run_amplifiers(
             [2, 3, 4, 0, 1],
             [3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33] ++
               [1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]
           ) ===
             65_210
  end

  test "process part 1" do
    assert Day7.process_part1(Util.get_input()) === 422_858
  end

  @tag :skip
  test "process part 2" do
    assert Day7.process_part2(Util.get_input()) === :answer
  end
end

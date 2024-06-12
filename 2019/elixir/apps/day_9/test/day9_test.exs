defmodule Day9Test do
  use ExUnit.Case
  doctest Day9, import: true

  test "process part 1" do
    assert Day9.process_part1(Util.get_input()) === 3_906_448_201
  end

  test "process part 2" do
    assert Day9.process_part2(Util.get_input()) === 59_785
  end
end

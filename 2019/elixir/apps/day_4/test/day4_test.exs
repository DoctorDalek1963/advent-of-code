defmodule Day4Test do
  use ExUnit.Case
  doctest Day4, import: true

  test "process part 1" do
    assert Day4.process_part1(Util.get_input()) === 544
  end

  test "process part 2" do
    assert Day4.process_part2(Util.get_input()) === 334
  end
end

defmodule Day4Test do
  use ExUnit.Case
  doctest Day4

  test "process part 1" do
    assert Day4.process_part1(Util.get_real_input()) === 544
  end

  test "process part 2" do
    assert Day4.process_part2(Util.get_real_input()) === 334
  end
end

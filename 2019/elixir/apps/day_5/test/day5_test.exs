defmodule Day5Test do
  use ExUnit.Case
  doctest Day5, import: true

  test "process part 1" do
    assert Day5.process_part1(Util.get_real_input()) === 11_049_715
  end

  @tag :skip
  test "process part 2" do
    assert Day5.process_part2(Util.get_real_input()) === 0
  end
end
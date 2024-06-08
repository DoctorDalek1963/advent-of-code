defmodule Day2Test do
  use ExUnit.Case
  doctest Day2, import: true

  test "process part 1" do
    assert Day2.process_part1(Util.get_real_input()) === 4_138_687
  end

  test "process part 2" do
    assert Day2.process_part2(Util.get_real_input()) === 6635
  end
end

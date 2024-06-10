defmodule Day6Test do
  use ExUnit.Case
  doctest Day6, import: true

  test "process part 1" do
    assert Day6.process_part1(Util.get_input()) === 292_387
  end

  test "process part 2" do
    assert Day6.process_part2(Util.get_input()) === 433
  end
end

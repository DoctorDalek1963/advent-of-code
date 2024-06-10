defmodule DayDAYNUMTest do
  use ExUnit.Case
  doctest DayDAYNUM, import: true

  @tag :skip
  test "process part 1" do
    assert DayDAYNUM.process_part1(Util.get_input()) === :answer
  end

  @tag :skip
  test "process part 2" do
    assert DayDAYNUM.process_part2(Util.get_input()) === :answer
  end
end

defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  defp get_test_input() do
    """
      12
      14
      1969
      100756
    """
  end

  test "process part 1" do
    assert Day1.process_part1(get_test_input()) === 34241
    assert Day1.process_part1(Util.get_real_input()) === 3_372_756
  end

  test "process part 2" do
    assert Day1.process_part2(get_test_input()) === 51316
    assert Day1.process_part2(Util.get_real_input()) === 5_056_279
  end
end